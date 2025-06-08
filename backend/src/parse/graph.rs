use neo4rs::{query, Graph, Config, ConfigBuilder};
use serde_json::Value;
use std::{collections::HashMap, error::Error};
use crate::cypher::cquery::CQueryStorage;

#[derive(Clone)]
pub struct GeneralGraph {
    pub graph: Graph,
    pub queries: CQueryStorage
}

impl GeneralGraph {
    /* TODO: Make only one config. */
    pub async fn new(db_addr: &str, db_name: &str, username: &str, password: &str) -> Result<Self, Box<dyn Error>> {
        let queries = CQueryStorage::new();

        let config = ConfigBuilder::default()
            .uri(db_addr)
            .user(username)
            .password(password)
            .db(db_name)
            .fetch_size(500)
            .max_connections(10)
            .build()?; // propagate build error

        let graph = Graph::connect(config).await?; // propagate connection error

        Ok(Self{ graph, queries })
    }

    pub async fn connect(&mut self, db_addr: &str, db_name: &str, username: &str, password: &str) -> Result<(), Box<dyn Error>> {
        let config = ConfigBuilder::default()
            .uri(db_addr)
            .user(username)
            .password(password)
            .db(db_name)
            .fetch_size(500)
            .max_connections(10)
            .build()?; // propagate build error

        let graph = Graph::connect(config).await?; // propagate connection error

        self.test_connection().await?;

        self.graph = graph;

        Ok(())
    }

    pub async fn test_connection(&mut self) -> Result<(), Box<dyn Error>> {
        let mut result = self.graph.execute(query("RETURN 1")).await?;
        while let Ok(Some(_)) = result.next().await {}
        Ok(())
    }


    pub async fn run(&self, q: &str) -> Result<(), neo4rs::Error> {
        println!("About to run: {}", q);
        self.graph.run(query(q))
        .await
        .map_err(|e| {
            eprintln!("Graph Query Failed on Run: {:?}", e);
            e
        })
    }

    pub async fn create_node(&self, category: &str, label_id: &str, label_val: &str) -> Result<(), Box<dyn Error>> {
        let create_node_query = self.queries.get_create_node(
            category, label_id, label_val);
        println!("Creating Node: {}", create_node_query.query);
        self.run(&create_node_query.query).await?;
        println!("Finished running.");
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn delete_node(&self, category: &str, label_id: &str, label_val: &str) -> Result<(), Box<dyn Error>> {
        let remove_node_query = self.queries.get_delete_node(category, label_id, label_val);
        self.run(&remove_node_query.query).await?;
        Ok(())
    }

    pub async fn set_node_property(&self, category: &str, label_id: &str, label_val: &str, property_key: &str, property_val: &str) -> Result<(), Box<dyn Error>> {
        let set_node_property = self.queries.get_set_node_property(
            category, label_id, label_val, property_key, property_val);

        println!("Running Query: {}", set_node_property.name);
        self.run(&set_node_property.query).await?;
        Ok(())
    }

    pub async fn set_relationship(&self, cat_a: &str, node_a_key: &str, node_a_val: &str, 
        cat_b: &str, node_b_key: &str, node_b_val: &str, 
        cat_rel: &str, prop_key: &str, prop_val: &str) -> Result<(), Box<dyn Error>> {
        let set_relationship = self.queries.get_set_relationship(
            cat_a, node_a_key, node_a_val, 
            cat_b, node_b_key, node_b_val, 
            cat_rel, prop_key, prop_val);
        println!("Running Query: {}", set_relationship.query);
        self.run(&set_relationship.query).await?;
        Ok(())
    }

    pub async fn set_relationship_property(&self, 
        rel_id: &str, rel_key: &str, 
        prop_key: &str, prop_val: &str) -> Result<(), Box<dyn Error>> {
        let set_relationship = self.queries.get_set_relationship_property(rel_id, rel_key, prop_key, prop_val);
        self.run(&set_relationship.query).await?;
        Ok(())
    }

    pub async fn remove_relationship(&self, cat_a: &str, node_a_key: &str, node_a_val: &str, 
        cat_b: &str, node_b_key: &str, node_b_val: &str, 
        cat_rel: &str, prop_key: &str, prop_val: &str) -> Result<(), Box<dyn Error>> {
        let delete_rel = self.queries.get_delete_relationship(cat_a, node_a_key, node_a_val, cat_b, node_b_key, node_b_val, cat_rel, prop_key, prop_val);
        self.run(&delete_rel.query).await?;
        Ok(())
    }

    pub async fn get_possible_paths(&self, start_state: &str, end_state: &str) -> 
    // Result<Vec<HashMap<String,Vec<String>>>, neo4rs::Error> {
    Vec<String> {
        let possible_paths_q = self.queries.get_possible_pathways(start_state, end_state);
        let mut result = self.graph.execute(query(&possible_paths_q.query)).await.unwrap();

        let mut paths = Vec::new();

        while let Some(row) = result.next().await.transpose() {
            let row = row.unwrap();
            // let states: Vec<String> = row.get::<Vec<String>>("states").unwrap();
            let lexical_items: Vec<String> = row.get::<Vec<String>>("items").unwrap();
            let full_path = lexical_items.join(" => ");
            
            paths.push(full_path);
        }
        paths
    }

    pub async fn get_shortest_paths(&self, start_state: &str, end_state: &str) -> 
        Vec<String> {
        let shortest_paths_q = self.queries.get_shortest_pathways(start_state, end_state);
        let mut result = self.graph.execute(query(&shortest_paths_q.query)).await.unwrap();
        let mut paths = Vec::new();
        while let Some(row) = result.next().await.transpose() {
            let row = row.unwrap();
            // let states: Vec<String> = row.get::<Vec<String>>("states").unwrap();
            let lexical_items: Vec<String> = row.get::<Vec<String>>("items").unwrap();
            let full_path = lexical_items.join(" => ");
            paths.push(full_path);
        }
        paths
    }

    /* Empties the Graph Database */
    pub async fn clear(&self) -> Result<(), neo4rs::Error> {
        let clear_graph_query: &str = &self.queries.get_clear_graph().query;
        self.run(clear_graph_query).await
    }

    /* Removes redundancy. */
    pub async fn remove_redundant_nodes(&self) -> Result<(), neo4rs::Error> {
        let remove_redundant_nodes_query: &str = &self.queries.get_remove_redundant_nodes().query;
        self.run(remove_redundant_nodes_query).await
    }
}

#[allow(dead_code)]
pub struct State<'a> {
    state_id: &'a str,
}

pub struct Edge<'a> {
    pub state_a_id: &'a str,
    pub state_b_id: &'a str,
    pub rel: &'a str
}

#[derive(Clone)]
pub struct GrammarGraph {
    pub base: GeneralGraph,
}

impl GrammarGraph {
    pub async fn new(
        db_addr: &str, 
        db_name: &str, 
        username: &str, 
        password: &str
    ) -> Result<Self, Box<dyn Error>> {
        let base = GeneralGraph::new(db_addr, db_name, username, password).await?;
        Ok(Self { base })
    }

    pub async fn connect(&mut self, db_addr: &str, db_name: &str, db_user: &str, db_pw: &str) -> Result<(), Box<dyn Error>>{
        self.base.connect(db_addr, db_name, db_user, db_pw).await
    }

    pub async fn test_connection(&mut self) -> Result<(), Box<dyn Error>>{
        self.base.test_connection().await
    }

    pub async fn set_state_property(&self, label_id: &str, label_val: &str, prop_key: &str, prop_val: &str) -> Result<(), Box<dyn Error>>{
        println!("Setting State Property");
        self.base.set_node_property("State", label_id, label_val, prop_key, prop_val).await?;
        Ok(())
    }

    /*
    TODO: Fix for cases where the LI is the same.
    */
    pub async fn set_merge_property(&self, li_morph: &str, prop_key: &str, prop_val: &str) -> Result<(), Box<dyn Error>>{
        println!("Setting Relationship Property");
        self.base.set_relationship_property("li", li_morph, prop_key, prop_val).await?;
        Ok(())
    }

    pub async fn create_state(&self, name: &str, type_: Option<&str>) -> Result<(), Box<dyn Error>> {
        self.base.create_node(type_.unwrap_or("State"), "name", name).await?;
        self.set_state_property("name", name, "move", "").await?;
        Ok(())
    }

    // "MATCH (a:{} {{ name: \"{}\" }})-[edge:MERGE {{ li: \'{}\' }}]->(b:{} {{name: \"{}\" }}) DELETE edge"
    pub async fn connect_states(&self, state_a: &str, state_b: &str, rel: &str, 
        state_a_type_: Option<&str>, state_b_type_: Option<&str>) -> Result<(), Box<dyn Error>> {
        self.base.set_relationship(state_a_type_.unwrap_or("State"), "name", state_a, 
        state_b_type_.unwrap_or("State"), "name", state_b, "MERGE", "li", rel).await?;
        
        // NOTE: Fix for relationships of the same LI
        self.set_merge_property(rel, "move", "").await?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn delete_edge<'a>(&self, edge: &Edge<'a>) -> Result<(), Box<dyn Error>> {
        self.base.remove_relationship("State", "name", &edge.state_a_id, 
        "State", "name", &edge.state_b_id, "MERGE", "li", &edge.rel).await?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn contract_edge<'a>(&self, edge: &Edge<'a>) -> Result<(), Box<dyn Error>> {
        println!("Contracting {}-{}-{}", edge.state_a_id, edge.state_b_id, edge.rel);
        let new_node_id = format!("{}-{}", edge.state_a_id, edge.state_b_id);
        let contract_edge_query = format!(
            "MATCH (a:State {{ name: '{}' }})-[e:MERGE {{ li: \'{}\' }}]->(b:State {{ name: '{}' }})
                WITH a, b, e
                CREATE (merged:State {{ name: '{}' }})
                DELETE e", 
            &edge.state_a_id, &edge.rel, &edge.state_b_id, &new_node_id);

        println!("Query: {}", contract_edge_query);
        self.base.graph.run(query(contract_edge_query.as_str())).await?;

        let reassign_relationships_from_new_node = format!(
            "MATCH (a)-[r:MERGE]->(b)
            WHERE a.name = '{}' OR a.name = '{}' 
            WITH a, b, r
            MATCH (n {{ name: '{}' }})
            CREATE (n)-[newRel: MERGE {{ li: r.li }}]->(b)", 
            &edge.state_a_id, &edge.state_b_id, &new_node_id);
        println!("Query: {}", reassign_relationships_from_new_node);
        self.base.graph.run(query(&reassign_relationships_from_new_node.as_str())).await?;

        let reassign_relationships_to_new_node = format!(
            "MATCH (a)-[r:MERGE]->(b)
            WHERE b.name = '{}' OR b.name = '{}' 
            WITH a, b, r
            MATCH (n {{ name: '{}' }})
            CREATE (a)-[newRel: MERGE {{ li: r.li }}]->(n)", 
            &edge.state_a_id, &edge.state_b_id, &new_node_id);
        println!("Query: {}", reassign_relationships_to_new_node);
        self.base.graph.run(query(&reassign_relationships_to_new_node.as_str())).await?;

        self.base.delete_node("State", "name", &edge.state_a_id).await?;
        self.base.delete_node("State", "name", &edge.state_b_id).await?;
        Ok(())
    }

    pub async fn get_possible_paths(&self) -> Result<Vec<String>, neo4rs::Error> {
        let start_state: &str = "d";
        let end_state: &str = "t";
        Ok(self.base.get_possible_paths(start_state,end_state).await)
    }

    pub async fn get_shortest_paths(&self) -> Result<Vec<String>, neo4rs::Error> {
        let start_state: &str = "d";
        let end_state: &str = "t";
        Ok(self.base.get_shortest_paths(start_state,end_state).await)
    }

    pub async fn clear(&self) -> Result<(), neo4rs::Error> {
        self.base.clear().await
    }

    pub async fn remove_redundancy(&self) -> Result<(), neo4rs::Error> {
        println!("Removing redundancy from graph.");
        self.base.remove_redundant_nodes().await
    }

}
