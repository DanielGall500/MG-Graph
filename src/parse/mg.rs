use neo4rs::{query, Graph};
use std::error::Error;
use std::io::Write;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::collections::HashSet;
use crate::cypher::cquery::CQueryStorage;


pub struct GeneralGraph {
    graph: Graph,
    queries: CQueryStorage
}

impl GeneralGraph {
    pub async fn new(db_id: &str, username: &str, password: &str) -> Result<Self, Box<dyn Error>> {
        let graph = Graph::new(db_id, username, password).await?;
        let queries = CQueryStorage::new();

        println!("{}", username);
        println!("{}", password);
        println!("Connected to database.");

        Ok(Self{ graph, queries })
    }

    pub async fn run(&self, q: &str) -> Result<(), neo4rs::Error> {
        println!("About to run: {}", q);
        self.graph.run(query(q))
        .await
        .map_err(|e| {
            eprintln!("Graph Query Failed: {:?}", e);
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

    #[allow(dead_code)]
    pub async fn remove_relationship(&self, cat_a: &str, node_a_key: &str, node_a_val: &str, 
        cat_b: &str, node_b_key: &str, node_b_val: &str, 
        cat_rel: &str, prop_key: &str, prop_val: &str) -> Result<(), Box<dyn Error>> {
        let delete_rel = self.queries.get_delete_relationship(cat_a, node_a_key, node_a_val, cat_b, node_b_key, node_b_val, cat_rel, prop_key, prop_val);
        self.run(&delete_rel.query).await?;
        Ok(())
    }

    /* Empties the Graph Database */
    pub async fn clear(&self) -> Result<(), neo4rs::Error> {
        let clear_graph_query: &str = &self.queries.get_clear_graph().query;
        self.run(clear_graph_query).await
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
        self.base.create_node("State", "name", name).await?;
        Ok(())
    }

    // "MATCH (a:{} {{ name: \"{}\" }})-[edge:MERGE {{ li: \'{}\' }}]->(b:{} {{name: \"{}\" }}) DELETE edge"
    pub async fn connect_states(&self, state_a: &str, state_b: &str, rel: &str) -> Result<(), Box<dyn Error>> {
        self.base.set_relationship("State", "name", state_a, 
        "State", "name", state_b, "MERGE", "li", rel).await?;
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
    pub mg: Vec<LexicalItem>,
    pub states: HashSet<String>
}

#[derive(Serialize, Deserialize)]
pub enum LIRelation {
    LMerge, // =x
    RMerge, // x= 
    LMergeHead, // =>x
    RMergeHead, // x<=
    MinusMove, // -x
    PlusMove, // +x
    State, // x
}

// #[derive(Debug)]
impl MGParser {
    pub fn new() -> Self {
        Self{
            mg: Vec::new(),
            states: HashSet::new(),
        }
    }

    pub async fn create_grammar_graph(&mut self, gg: &GrammarGraph) -> Result<(), Box<dyn Error>> {
        let mut merge_state: Option<&Feature>;
        let mut final_state: Option<&Feature>;
        let mut move_hoover: Option<&Feature>;
        let mut bundle: &Vec<Feature>;
        let mut is_head: bool;

        for li in &self.mg {
            // check if this new lexical item is a head or not
            merge_state = None;
            final_state = None;
            move_hoover = None;
            bundle = &li.bundle;

            // if the first feature is left or right merge, the LI is a head
            if let Some(first_feature) = bundle.first() {
                is_head = matches!(first_feature.rel, LIRelation::LMerge) || matches!(first_feature.rel, LIRelation::RMerge);
            }
            else {
                eprintln!("LI Contains No Features: {}", li.morph);
                continue;
            }
            

            for f in bundle.iter() {
                match f.rel {
                    LIRelation::LMerge | 
                    LIRelation::RMerge | 
                    LIRelation::LMergeHead | 
                    LIRelation::RMergeHead => merge_state = Some(f),

                    LIRelation::PlusMove | 
                    LIRelation::MinusMove => move_hoover = Some(f),

                    LIRelation::State => final_state = Some(f),
                }

                // STEP: ADD ANY STATES (NODES) FROM FEATURE
                // any states found through selectional or categorial features
                // are added as states to our MG
                if matches!(f.rel, LIRelation::LMerge | LIRelation::State | LIRelation::RMerge) && !self.states.contains(&f.id) {
                    self.states.insert(f.id.to_string());
                    gg.create_state(&f.id.as_str()).await?;
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
        let mut file = File::create("./grammar/grammar_parsed.json")?;
        match serde_json::to_string_pretty(&self.mg) {
            Ok(json) => file.write_all(json.as_bytes())?,
            Err(e) => eprintln!("Error serializing data to JSON: {}", e),
        }
        Ok(())
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
            li = LexicalItem { 
                morph: String::from(""), 
                bundle: Vec::new() 
            };

            // e.g laughs :: d= +k t
            let morph_feature_split: Vec<String> = l.split("::").map(|c| c.to_string()).collect();

            // STEP 1: process the phonological form: e.g "Mary" in "Mary" :: d -k 
            if let Some(morph) = morph_feature_split.get(0) {
                li.morph = morph.trim().to_string();
                println!("Valid Morph: {}", li.morph);
            }
            else {
                eprintln!("Invalid MG Statement: {}", l);
                eprintln!("Error was found in phonological form parsing.")
            }

            // STEP 2: parse the feature bundle e.g 'd -k' in "Mary" :: d -k
            if let Some(features) = morph_feature_split.get(1) {
                let individual_feature_split = features
                    .split_whitespace()
                    .map(|c| c.trim().to_string());

                // STEP 3: iterate over each feature in the LI and add
                // to the feature bundle
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

                    // STEP 4: ADD FEATURE INFO TO LI
                    li.bundle.push(Feature {
                        raw: feature.clone(),
                        id: id.clone(),
                        rel: relation,
                    });

                    println!("Valid -{}-", feature.to_string());
                }
            }
            else {
                eprintln!("Invalid MG Statement: {}", l);
                eprint!("Error was found during feature bundle parsing.")
            }
            self.mg.push(li);
        }
        Ok(())
    }
}
