use neo4rs::{query, Graph, ConfigBuilder};
use std::{error::Error};
use crate::cypher::cquery::{CQueryStorage, Node, Relationship};
use std::collections::HashMap;

#[derive(Clone)]
pub struct GeneralGraph {
    pub graph: Graph,
    pub queries: CQueryStorage
}

const DEFAULT_NODE_STATE: &str = "State";
const INTERM_NODE_STATE: &str = "Interm";

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

    pub async fn create_node(&self, n: Node) -> Result<(), Box<dyn Error>> {
        let create_node_query = self.queries.get_create_node(n);
        println!("Creating Node: {}", create_node_query.query);
        self.run(&create_node_query.query).await?;
        println!("Finished running.");
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn delete_node(&self, n: Node) -> Result<(), Box<dyn Error>> {
        let remove_node_query = self.queries.get_delete_node(n);
        self.run(&remove_node_query.query).await?;
        Ok(())
    }

    pub async fn set_node_property(&self, category: &str, label_val: &str, property_key: &str, property_val: &str) -> Result<(), Box<dyn Error>> {
        let set_node_property = self.queries.get_set_node_property(
            category, "name", label_val, property_key, property_val);

        println!("Running Query: {}", set_node_property.name);
        self.run(&set_node_property.query).await?;
        Ok(())
    }

    pub async fn set_relationship(&self, rel: Relationship) -> Result<(), Box<dyn Error>> {
        let set_relationship = self.queries.get_set_relationship(rel.clone());
        println!("Running Query: {}", set_relationship.query);
        self.run(&set_relationship.query).await?;

        self.set_relationship_property("li", rel.li.as_str(), "move", "").await?;
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

    pub async fn contract_edge(&self, node_a_label: &str, node_b_label: &str) -> Result<(), Box<dyn Error>> {
        let contract_edge = self.queries.get_contract_edge(node_a_label, node_b_label);
        self.run(&contract_edge.query).await?;
        Ok(())
    }

    pub async fn switch_edge_origin(&self, node_label_prev_origin: &str, node_label_new_origin: &str) -> Result<(), Box<dyn Error>> {
        let switch_edge_origin = self.queries.get_switch_edge_origin(node_label_prev_origin, node_label_new_origin);
        self.run(&switch_edge_origin.query).await?;
        Ok(())
    }

    pub async fn switch_edge_endpoint(&self, node_label_prev_endpoint: &str, node_label_new_endpoint: &str) -> Result<(), Box<dyn Error>> {
        let switch_edge_endpoint = self.queries.get_switch_edge_endpoint(node_label_prev_endpoint, node_label_new_endpoint);
        self.run(&switch_edge_endpoint.query).await?;
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

    pub async fn set_state_property(&self, label_val: &str, prop_key: &str, prop_val: &str) -> Result<(), Box<dyn Error>>{
        println!("Setting State Property");
        self.base.set_node_property("State",  label_val, prop_key, prop_val).await?;
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

    pub async fn create_state(&self, n: Node) -> Result<(), Box<dyn Error>> {
        self.base.create_node(n).await?;
        Ok(())
    }

    // "MATCH (a:{} {{ name: \"{}\" }})-[edge:MERGE {{ li: \'{}\' }}]->(b:{} {{name: \"{}\" }}) DELETE edge"
    pub async fn connect_states(&self, rel: Relationship) -> Result<(), Box<dyn Error>> {
        self.base.set_relationship(rel).await?;
        
        // NOTE: Fix for relationships of the same LI
        // self.set_merge_property(rel.li.as_str(), "move", "").await?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn delete_edge<'a>(&self, edge: &Edge<'a>) -> Result<(), Box<dyn Error>> {
        self.base.remove_relationship("State", "name", edge.state_a_id, 
        "State", "name", edge.state_b_id, "MERGE", "li", edge.rel).await?;
        Ok(())
    }

    /*
    The below code should be far improved for SOC
    */
    #[allow(dead_code)]
    pub async fn contract_edge<'a>(&self, node_a: &str, node_b: &str) -> Result<(), Box<dyn Error>> {
        let mut basic_rel_props : HashMap<String, String> = HashMap::new();
        basic_rel_props.insert(String::from("move"), String::from(""));
        let rel: Relationship = Relationship {
            node_a: Node {
                state_type: String::from("State"),
                label: node_a.to_string(),
                props: None
            },
            node_b: Node {
                state_type: String::from("State"),
                label: node_b.to_string(),
                props: None
            },
            li: String::from(""),
            props: basic_rel_props,
        };
        let new_node_label = format!("{}-{}", rel.node_a.label, rel.node_b.label);
        self.base.contract_edge(node_a, node_b).await
           .map_err(|e| format!("contract_edge failed: {}", e))?;
        self.base.create_node(Node {
            state_type: String::from("State"),
            label: new_node_label.clone(),
            props: None
        }).await
            .map_err(|e| format!("create new node failed: {}", e))?;

        self.base.switch_edge_origin(node_a, &new_node_label).await
            .map_err(|e| format!("switch edge origin A failed: {}", e))?;
        self.base.switch_edge_endpoint(node_a, &new_node_label).await
            .map_err(|e| format!("switch edge endpoint A failed: {}", e))?;

        self.base.switch_edge_origin(node_b, &new_node_label).await
            .map_err(|e| format!("switch edge origin B failed: {}", e))?;
        self.base.switch_edge_endpoint(node_b, &new_node_label).await
            .map_err(|e| format!("switch edge endpoint B failed: {}", e))?;

        self.base.delete_node(rel.node_a.clone()).await
            .map_err(|e| format!("delete node A failed: {}", e))?;

        self.base.delete_node(rel.node_b.clone()).await
            .map_err(|e| format!("delete node B failed: {}", e))?;
        Ok(())
    }

    pub async fn get_possible_paths(&self, start_item: &str, end_item: &str) -> Result<Vec<String>, neo4rs::Error> {
        Ok(self.base.get_possible_paths(start_item, end_item).await)
    }

    pub async fn get_shortest_paths(&self, start_item: &str, end_item: &str) -> Result<Vec<String>, neo4rs::Error> {
        Ok(self.base.get_shortest_paths(start_item, end_item).await)
    }

    pub async fn clear(&self) -> Result<(), neo4rs::Error> {
        self.base.clear().await
    }

    pub async fn remove_redundancy(&self) -> Result<(), neo4rs::Error> {
        println!("Removing redundancy from graph.");
        self.base.remove_redundant_nodes().await
    }

}
