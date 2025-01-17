use neo4rs::{query, Graph};
use core::panic;
use std::error::Error;
use std::io::{Write, Read};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::fs::File;
use std::collections::HashSet;

pub struct GeneralGraph {
    graph: Graph,
}

impl GeneralGraph {
    pub async fn new(db_id: &str, username: &str, password: &str) -> Result<Self, Box<dyn Error>> {
        let graph = Graph::new(db_id, username, password).await?;
        println!("{}", username);
        println!("{}", password);
        println!("Connected to database.");
        Ok(Self{ graph })
    }

    pub async fn create_node(&self, category: &str, label_id: &str, label_val: &str) -> Result<(), Box<dyn Error>> {
        let create_node_query = format!("CREATE (p:{} {{ {}: \"{}\" }})", category, label_id, label_val);
        self.graph.run(query(create_node_query.as_str())).await?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn delete_node(&self, category: &str, label_id: &str, label_val: &str) -> Result<(), Box<dyn Error>> {
        let remove_node_query = format!("MATCH (p:{} {{ {}: \"{}\" }}) DETACH DELETE p", category, label_id, label_val);
        self.graph.run(query(&remove_node_query)).await?;
        Ok(())
    }

    pub async fn set_node_property(&self, category: &str, node_id_key: &str, node_id_val: &str, property_key: &str, property_val: &str) -> Result<(), Box<dyn Error>> {
        let set_node_property_query = format!(
            "MATCH (n:{} {{ {}: \"{}\" }}) SET n.{} = \"{}\"; ",
            category, node_id_key, node_id_val, property_key, property_val
        );
        println!("Running Node Property: {}", set_node_property_query);
        self.graph.run(query(set_node_property_query.as_str())).await?;
        Ok(())
    }

    pub async fn set_relationship_property(&self, rel_id: &str, rel_val: &str, property_key: &str, property_val: &str) -> Result<(), Box<dyn Error>> {
        let set_rel_property_query: String = format!(
            "MATCH ()-[r]->() WHERE r.{} = \"{}\" SET r.{} = \"{}\"; ",
            rel_id, rel_val, property_key, property_val
        );
        println!("Running Rel Property: {}", set_rel_property_query);
        self.graph.run(query(set_rel_property_query.as_str())).await?;
        Ok(())
    }

    // use MERGE instead
    pub async fn set_relationship(&self, cat_a: &str, cat_b: &str, node_a: &str, node_b: &str, rel: &str) -> Result<(), Box<dyn Error>> {
        println!("Creating {}-{}-{}", node_a, node_b, rel);
        let set_relationship_query = format!(
            "MATCH (a:{} {{ name: \"{}\" }}), (b:{} {{name: \"{}\" }})
            CREATE (a)-[:MERGE {{ li: \'{}\' }}]->(b)
            RETURN a, b", cat_a, node_a, cat_b, node_b, rel);
        self.graph.run(query(set_relationship_query.as_str())).await?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn remove_relationship(&self, cat_a: &str, cat_b: &str, node_a: &str, node_b: &str, rel: &str) -> Result<(), Box<dyn Error>> {
        println!("Removing {}-{}-{}", node_a, node_b, rel);
        let delete_relationship_query = format!(
            "MATCH (a:{} {{ name: \"{}\" }})-[edge:MERGE {{ li: \'{}\' }}]->(b:{} {{name: \"{}\" }})
            DELETE edge", 
            cat_a, node_a, rel, cat_b, node_b);
        self.graph.run(query(delete_relationship_query.as_str())).await?;
        Ok(())
    }

    /* Empties the Graph Database */
    pub async fn clear(&self) -> Result<(), neo4rs::Error> {
        let remove_all_query: &str = "MATCH (n) DETACH DELETE n";
        self.graph.run(query(remove_all_query))
        .await
        .map(|e| {
            eprintln!("Graph Clearing Failed: {:#?}", e);
            e
        })
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

pub struct GrammarGraph {
    base: GeneralGraph,
}

impl GrammarGraph {
    pub async fn new(
        db_id: &str, 
        username: &str, 
        password: &str
    ) -> Result<Self, Box<dyn Error>> {
        let base = GeneralGraph::new(db_id, username, password).await?;
        Ok(Self { base })
    }

    pub async fn set_state_property(&self, label_id: &str, label_val: &str, prop_key: &str, prop_val: &str) -> Result<(), Box<dyn Error>>{
        println!("Setting State Property");
        self.base.set_node_property("State", label_id, label_val, prop_key, prop_val).await?;
        Ok(())
    }

    pub async fn set_merge_property(&self, li_morph: &str, prop_key: &str, prop_val: &str) -> Result<(), Box<dyn Error>>{
        println!("Setting Relationship Property");
        self.base.set_relationship_property("li", li_morph, prop_key, prop_val).await?;
        Ok(())
    }

    pub async fn create_state(&self, name: &str) -> Result<(), Box<dyn Error>> {
        // let properties = format!("{{name: '{}'}}", name);
        self.base.create_node("State", "name", name).await?;
        Ok(())
    }

    pub async fn connect_states(&self, state_a: &str, state_b: &str, rel: &str) -> Result<(), Box<dyn Error>> {
        self.base.set_relationship("State", "State", state_a, state_b, rel).await?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn delete_edge<'a>(&self, edge: &Edge<'a>) -> Result<(), Box<dyn Error>> {
        self.base.remove_relationship("State", "State", 
        &edge.state_a_id, &edge.state_b_id, &edge.rel).await?;
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

    pub async fn clear(&self) -> Result<(), neo4rs::Error> {
        self.base.clear().await
    }

}

#[derive(Serialize, Deserialize)]
pub struct LexicalItem {
    pub morph: String,
    pub bundle: Vec<Feature>,
}
#[derive(Serialize, Deserialize)]
pub struct Feature {
    pub raw: String,
    pub id: String,
    pub rel: LIRelation
}

pub struct MGParser {
    states: HashSet<String>,
    pub mg: Vec<LexicalItem>
}

#[derive(Serialize, Deserialize)]
pub enum LIRelation {
    LMerge,
    RMerge,
    MinusMove,
    PlusMove,
    State,
}

#[derive(Debug)]
pub struct CQuery {
    name: String,
    query: String,
    desc: String,
}


impl MGParser {
    pub fn new() -> Self {
        Self{
            states: HashSet::new(),
            mg: Vec::new(),
        }
    }

    pub async fn create_grammar_graph(&mut self, gg: &GrammarGraph) -> Result<(), Box<dyn Error>> {
        let mut merge_state: Option<&Feature>;
        let mut final_state: Option<&Feature>;
        let mut move_hoover: Option<&Feature>;
        let mut is_head: bool;

        // first handle LIs with no selectional features
        for li in &self.states {
            /* Create nodes with the states */
            gg.create_state(li.as_str()).await?;
        }

        for li in &self.mg {
            // check if this new lexical item is a head or not
            merge_state = None;
            final_state = None;
            move_hoover = None;
            is_head = false;
            

            for (i, f) in li.bundle.iter().enumerate() {
                // if the first feature is left or right merge, the LI is a head
                if i == 0 {
                    is_head = matches!(f.rel, LIRelation::LMerge) || matches!(f.rel, LIRelation::RMerge);
                }

                match f.rel {
                    LIRelation::LMerge => merge_state = Some(f),
                    LIRelation::RMerge => merge_state = Some(f),
                    LIRelation::PlusMove => move_hoover = Some(f),
                    LIRelation::MinusMove => move_hoover = Some(f),
                    LIRelation::State => final_state = Some(f),
                }
            }

            println!("Connecting States");
            // there should at least be a final state, either one it becomes after feature checking
            // or one that it currently is with leftover features
            if let Some(state_b) = final_state.take() {
                println!("Setting Property For {}", li.morph);

                // create a relationship between a potential state A and B (handles LIs with selectional features)
                if let Some(state_a) = merge_state.take() {
                    gg.connect_states(&state_a.id, &state_b.id, &li.morph).await?;
                }

                // attach any movement features to the newly created relationship
                if let Some(movement) = move_hoover.take() {
                    if is_head {
                        // heads are represented as a relationship and as such the property
                        // of a relationship is set
                        gg.set_merge_property(&li.morph, "move", &movement.raw).await?;
                    }
                    else {
                        // non-heads are represented as a state and as such the property
                        // of a state / node is set
                        gg.set_state_property("name", &state_b.raw, "move", &movement.raw).await?;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn to_json(&self) -> Result<(), Box<dyn Error>> {
        let mut file = File::create("output.json")?;
        match serde_json::to_string_pretty(&self.mg) {
            Ok(json) => file.write_all(json.as_bytes())?,
            Err(e) => eprintln!("Error serializing data to JSON: {}", e),
        }
        Ok(())
    }

    pub fn from_json(&self) -> Result<Vec<CQuery>, Box<dyn Error>> {
        let mut file = File::open("queries.json")?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let json: Value = serde_json::from_str(&content)?;

        let mut queries: Vec<CQuery> = Vec::new();

        if let Some(query_map) = json.get("queries").and_then(|q| q.as_object()) {
            for (_, value) in query_map {

                if let (Some(name), Some(query), Some(desc)) = (
                    value.get("name").and_then(|v| v.as_str()),
                    value.get("query").and_then(|v| v.as_str()),
                    value.get("desc").and_then(|v| v.as_str()),
                ) {
                    queries.push(CQuery {
                        query: query.to_string(),
                        name: name.to_string(),
                        desc: desc.to_string(),
                    });
                }
            }
        }
        Ok(queries)

    }

    pub fn parse_grammar_representation(&mut self, minimalist_grammar: &str) -> Result<(), Box<dyn Error>> {
        self.mg.clear();
        let mut li: LexicalItem;

        let mg_statements = minimalist_grammar
            .split(";")
            .into_iter()
            .filter(|l| { !l.is_empty() });

        for l in mg_statements {
            println!("LINE: {}", l);
            li = LexicalItem { morph: String::from(""), bundle: Vec::new() };

            // e.g laughs :: d= +k t
            let morph_feature_split: Vec<String> = l.split("::").map(|c| c.to_string()).collect();

            if let Some(morph) = morph_feature_split.get(0) {
                li.morph = morph.trim().to_string();
                println!("Morph: {}", li.morph);
            }
            else {
                panic!("Invalid MG file.");
            }

            if let Some(features) = morph_feature_split.get(1) {
                let individual_feature_split = features
                    .split_whitespace()
                    .map(|c| c.trim().to_string());

                for feature in individual_feature_split {
                    let (relation, id) = 
                    if feature.starts_with("=>") {
                        // need to create new relation for head merge
                        (LIRelation::LMerge, feature[2..].to_string())
                    }
                    else if feature.starts_with("=") {
                        (LIRelation::LMerge, feature[1..].to_string())
                    } else if feature.ends_with("=") {
                        (LIRelation::RMerge, feature[..feature.len() - 1].to_string())
                    } else if feature.starts_with("-") {
                        (LIRelation::MinusMove, feature[1..].to_string())
                    } else if feature.starts_with("+") {
                        (LIRelation::PlusMove, feature[1..].to_string())
                    } else {
                        (LIRelation::State, feature.clone())
                    };

                    if matches!(relation, LIRelation::LMerge | LIRelation::State | LIRelation::RMerge) {
                        self.states.insert(id.clone());
                    }

                    li.bundle.push(Feature {
                        raw: feature.clone(),
                        id: id.clone(),
                        rel: relation,
                    });

                    println!("-{}-", feature.to_string());
                }
            }
            self.mg.push(li);
        }
        Ok(())
    }
}
