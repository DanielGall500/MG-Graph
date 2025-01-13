use neo4rs::{query, Graph, Relation};
use core::panic;
use std::error::Error;
use std::io::Write;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::fs::write;
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

    pub async fn delete_node(&self, category: &str, label_id: &str, label_val: &str) -> Result<(), Box<dyn Error>> {
        let remove_node_query = format!("MATCH (p:{} {{ {}: \"{}\" }}) DETACH DELETE p", category, label_id, label_val);
        self.graph.run(query(&remove_node_query)).await?;
        Ok(())
    }

    pub async fn set_node_property(&self, node_type: &str, node_id_key: &str, node_id_val: &str, property_key: &str, property_val: &str) -> Result<(), Box<dyn Error>> {
        let set_node_property_query = format!(
            "MATCH (n:{} {{ {}: {} }}) SET n.{} = {} RETURN n",
            node_type, node_id_key, node_id_val, property_key, property_val
        );

        self.graph.run(query(set_node_property_query.as_str())).await?;
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

    pub async fn remove_relationship(&self, cat_a: &str, cat_b: &str, node_a: &str, node_b: &str, rel: &str) -> Result<(), Box<dyn Error>> {
        println!("Removing {}-{}-{}", node_a, node_b, rel);
        let delete_relationship_query = format!(
            "MATCH (a:{} {{ name: \"{}\" }})-[edge:MERGE {{ li: \'{}\' }}]->(b:{} {{name: \"{}\" }})
            DELETE edge", 
            cat_a, node_a, rel, cat_b, node_b);
        self.graph.run(query(delete_relationship_query.as_str())).await?;
        Ok(())
    }

    pub async fn clear(&self) -> Result<(), Box<dyn Error>> {
        let remove_all_query: &str = "MATCH (n) DETACH DELETE n";
        self.graph.run(query(remove_all_query)).await?;
        Ok(())
    }


}

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
    pub graph_title: String,
}

impl GrammarGraph {
    pub async fn new(
        db_id: &str, 
        username: &str, 
        password: &str,
        graph_title: &str) -> Result<Self, Box<dyn Error>> {
        let base = GeneralGraph::new(db_id, username, password).await?;
        Ok(Self { base, graph_title: graph_title.to_string() })
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

    pub async fn delete_edge<'a>(&self, edge: &Edge<'a>) -> Result<(), Box<dyn Error>> {
        self.base.remove_relationship("State", "State", 
        &edge.state_a_id, &edge.state_b_id, &edge.rel).await?;
        Ok(())
    }

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

    pub async fn clear(&self) -> Result<(), Box<dyn Error>> {
        self.base.clear().await?;
        Ok(())
    }

    pub fn get_title(&self) -> &str {
        &self.graph_title
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

impl MGParser {
    pub fn new() -> Self {
        Self{
            states: HashSet::new(),
            mg: Vec::new(),
        }
    }

    pub async fn create_grammar_graph(&mut self, gg: &GrammarGraph) -> Result<(), Box<dyn Error>> {
        for li in &self.states {
            /* Create nodes with the states */
            gg.create_state(li.as_str()).await?;
        }
        let mut merge_state: Option<&Feature> = None;
        let mut final_state: Option<&Feature> = None;
        for li in &self.mg {
            for f in &li.bundle {
                match f.rel {
                    LIRelation::LMerge => merge_state = Some(f),
                    LIRelation::RMerge => merge_state = Some(f),
                    LIRelation::PlusMove => println!("+ Move"),
                    LIRelation::MinusMove => println!("- Move"),
                    LIRelation::State => final_state = Some(f),
                }
            }

            if let Some(state_a) = merge_state.take() {
                if let Some(state_b) = final_state.take() {
                    gg.connect_states(&state_a.id, &state_b.id, &li.morph).await?;
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

    pub fn parse_grammar_representation(&mut self, minimalist_grammar: &str) -> Result<(), Box<dyn Error>> {
        self.mg.clear();
        let mut lexical_items: Vec<LexicalItem> = Vec::new();
        let mut lines = minimalist_grammar.split(";");
        for l in lines.into_iter() {
            let mut li: LexicalItem = LexicalItem { morph: String::from(""), bundle: Vec::new() };

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

                let mut relation: LIRelation = LIRelation::State;
                let mut id: String;

                for feature in individual_feature_split {
                    let (relation, id) = if feature.starts_with("=") {
                        (LIRelation::LMerge, feature[1..].to_string())
                    } else if feature.starts_with("=>") {
                        (LIRelation::LMerge, format!("-{}", &feature[2..]))
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
