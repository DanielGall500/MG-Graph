use serde_json::Value;
use core::panic;
use std::fs::File;
use std::error::Error;
use std::io::Read;
use std::collections::HashMap;

#[derive(Clone)]
pub struct CQuery {
    pub name: String,
    pub query: String,
    pub desc: String,
}

#[derive(Clone)]
pub struct CQueryStorage {
    pub queries: HashMap<String, CQuery>
}

const QUERIES_PATH: &str = "./cypher/queries.json";

impl CQueryStorage {
    pub fn new() -> Self {
        match load_queries_from_json(QUERIES_PATH) {
            Ok(queries) => Self { queries: queries },
            Err(e) => {
                eprintln!("Queries could not be loaded. Ensure the queries JSON file is available and formatted correctly");
                panic!("Error: {}", e);
            }
        }
    }

    pub fn get_query(&self, q_id: &str) -> &CQuery {
        self.queries.get(q_id).expect(&format!("No Query Available For ID {}", q_id))
    }

    pub fn get_clear_graph(&self) -> &CQuery {
        const Q_ID: &str = "clear_graph";
        self.get_query(Q_ID)
    }

    pub fn get_create_node(&self, state_type: &str, node_label_key: &str, node_label_val: &str) -> CQuery {
        const Q_ID: &str = "create_node";
        let q = self.get_query(Q_ID);

        CQuery { 
            name: q.name.clone(),
            query: q.query
                .replace("{TYPE}", state_type)
                .replace("{NODE_LABEL_KEY}", node_label_key)
                .replace("{NODE_LABEL_VAL}", node_label_val),
            desc: q.desc.clone()
        }
    }

    /* Yes, I know, code duplication. */
    pub fn get_delete_node(&self, state_type: &str, node_label_key: &str, node_label_val: &str) -> CQuery {
        const Q_ID: &str = "delete_node";
        let q = self.get_query(Q_ID);

        CQuery { 
            name: q.name.clone(),
            query: q.query
                .replace("{TYPE}", state_type)
                .replace("{NODE_LABEL_KEY}", node_label_key)
                .replace("{NODE_LABEL_VAL}", node_label_val),
            desc: q.desc.clone()
        }
    }

    pub fn get_set_node_property(&self, state_type: &str, node_label_key: &str, node_label_val: &str, property_key: &str, property_val: &str) -> CQuery {
        const Q_ID: &str = "set_node_property";
        let q = self.get_query(Q_ID);

        CQuery { 
            name: q.name.clone(),
            query: q.query
                .replace("{TYPE}", state_type)
                .replace("{NODE_LABEL_KEY}", node_label_key)
                .replace("{NODE_LABEL_VAL}", node_label_val)
                .replace("{PROPERTY_KEY}", property_key)
                .replace("{PROPERTY_VAL}", property_val),
            desc: q.desc.clone()
        }
    }

    pub fn get_set_relationship_property(&self, rel_id: &str, rel_val: &str, prop_key: &str, prop_val: &str) -> CQuery {
        const Q_ID: &str = "set_relationship_property";
        let q = self.get_query(Q_ID);

        CQuery { 
            name: q.name.clone(),
            query: q.query
                .replace("{REL_ID}", rel_id)
                .replace("{REL_VAL}", rel_val)
                .replace("{PROPERTY_KEY}", prop_key)
                .replace("{PROPERTY_VAL}", prop_val),
            desc: q.desc.clone()
        }
    }

    pub fn get_set_relationship(&self, node_a_type: &str, node_a_label_key: &str, node_a_label_val: &str, 
                                node_b_type: &str, node_b_label_key: &str, node_b_label_val: &str,
                                type_rel: &str, prop_key: &str, prop_val: &str) -> CQuery {

        const Q_ID: &str = "set_relationship";
        let q = self.get_query(Q_ID);

        CQuery { 
            name: q.name.clone(),
            query: q.query
                .replace("{NODE_A_TYPE}", node_a_type)
                .replace("{NODE_A_LABEL_KEY}", node_a_label_key)
                .replace("{NODE_A_LABEL_VAL}", node_a_label_val)
                .replace("{NODE_B_TYPE}", node_b_type)
                .replace("{NODE_B_LABEL_KEY}", node_b_label_key)
                .replace("{NODE_B_LABEL_VAL}", node_b_label_val)
                .replace("{REL_TYPE}", type_rel)
                .replace("{PROPERTY_KEY}", prop_key)
                .replace("{PROPERTY_VAL}", prop_val),
            desc: q.desc.clone()
        }
    }

    pub fn get_delete_relationship(&self, node_a_type: &str, node_a_label_key: &str, node_a_label_val: &str, 
                                node_b_type: &str, node_b_label_key: &str, node_b_label_val: &str,
                                type_rel: &str, prop_key: &str, prop_val: &str) -> CQuery {

        const Q_ID: &str = "delete_relationship";
        let q = self.get_query(Q_ID);

        CQuery { 
            name: q.name.clone(),
            query: q.query
                .replace("{NODE_A_TYPE}", node_a_type)
                .replace("{NODE_A_LABEL_KEY}", node_a_label_key)
                .replace("{NODE_A_LABEL_VAL}", node_a_label_val)
                .replace("{NODE_B_TYPE}", node_b_type)
                .replace("{NODE_B_LABEL_KEY}", node_b_label_key)
                .replace("{NODE_B_LABEL_VAL}", node_b_label_val)
                .replace("{REL_TYPE}", type_rel)
                .replace("{PROPERTY_KEY}", prop_key)
                .replace("{PROPERTY_VAL}", prop_val),
            desc: q.desc.clone()
        }
    }

}

pub fn load_queries_from_json(path: &str) -> Result<HashMap<String, CQuery>, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let json: Value = serde_json::from_str(&content)?;

    let mut queries: HashMap<String, CQuery> = HashMap::new();

    if let Some(query_map) = json.get("queries").and_then(|q| q.as_object()) {
        for (q_id, value) in query_map {

            if let (Some(name), Some(query), Some(desc)) = (
                value.get("name").and_then(|v| v.as_str()),
                value.get("query").and_then(|v| v.as_str()),
                value.get("desc").and_then(|v| v.as_str())
            ) {
                queries.insert(q_id.to_string(), 
                CQuery {
                    query: query.to_string(),
                    name: name.to_string(),
                    desc: desc.to_string(),
                });
            }
        }
    }
    else {
        println!("Just straight up dint work");
    }
    Ok(queries)
}
