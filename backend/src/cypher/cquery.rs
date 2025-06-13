use serde_json::Value;
use core::panic;
use std::error::Error;
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

#[derive(Clone)]
pub struct Node {
    pub state_type: String,
    pub label: String,
    pub props: Option<HashMap<String, String>>,
}

#[derive(Clone)]
pub struct Relationship {
    pub node_a: Node,
    pub node_b: Node,
    pub li: String,
    pub props: HashMap<String, String>,
}

const QUERIES_JSON: &str = include_str!("queries.json");

impl CQueryStorage {

    pub fn new() -> Self {
        match load_queries_from_json(QUERIES_JSON) {
            Ok(queries) => Self { queries },
            Err(e) => {
                eprintln!("Queries could not be loaded. Ensure the queries JSON file is available and formatted correctly");
                panic!("Error: {}", e);
            }
        }
    }

    pub fn get_query(&self, q_id: &str) -> &CQuery {
        self.queries.get(q_id).unwrap_or_else(|| panic!("No Query Available For ID {}", q_id))
    }

    pub fn get_remove_redundant_nodes(&self) -> &CQuery {
        const Q_ID: &str = "remove_redundant_nodes";
        self.get_query(Q_ID)
    }

    pub fn get_clear_graph(&self) -> &CQuery {
        const Q_ID: &str = "clear_graph";
        self.get_query(Q_ID)
    }

    pub fn get_create_node(&self, n: Node) -> CQuery {
        const Q_ID: &str = "create_node";
        let q = self.get_query(Q_ID);

        CQuery { 
            name: q.name.clone(),
            query: q.query
                .replace("{TYPE}", n.state_type.as_str())
                .replace("{NODE_LABEL_VAL}", n.label.as_str()),
            desc: q.desc.clone()
        }
    }

    /* Yes, I know, code duplication. */
    pub fn get_delete_node(&self, n: Node) -> CQuery {
        const Q_ID: &str = "delete_node";
        let q = self.get_query(Q_ID);

        CQuery { 
            name: q.name.clone(),
            query: q.query
                .replace("{TYPE}", n.state_type.as_str())
                .replace("{NODE_LABEL_VAL}", n.label.as_str()),
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

    pub fn get_set_relationship(&self, rel: Relationship) -> CQuery {

        

        const Q_ID: &str = "set_relationship";
        let q = self.get_query(Q_ID);

        // TODO: set props on relationshop
        CQuery { 
            name: q.name.clone(),
            query: q.query
                .replace("{NODE_A_TYPE}", rel.node_a.state_type.as_str())
                .replace("{NODE_A_LABEL_KEY}", "name")
                .replace("{NODE_A_LABEL_VAL}", rel.node_a.label.as_str())
                .replace("{NODE_B_TYPE}", rel.node_b.state_type.as_str())
                .replace("{NODE_B_LABEL_KEY}", "name")
                .replace("{NODE_B_LABEL_VAL}", rel.node_b.label.as_str())
                .replace("{REL_TYPE}", "Merge")
                .replace("{PROPERTY_KEY}", "li")
                .replace("{PROPERTY_VAL}", rel.li.as_str()),
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

    pub fn get_possible_pathways(&self, start_state: &str, end_state: &str) -> CQuery {
        const Q_ID: &str = "get_possible_paths";
        let q = self.get_query(Q_ID);

        CQuery {
            name: q.name.clone(),
            query: q.query
                .replace("{START_STATE}", start_state)
                .replace("{END_STATE}", end_state),
            desc: q.desc.clone()
        }
    }

    pub fn get_shortest_pathways(&self, start_state: &str, end_state: &str) -> CQuery {
        const Q_ID: &str = "get_shortest_paths";
        let q = self.get_query(Q_ID);

        CQuery {
            name: q.name.clone(),
            query: q.query
                .replace("{START_STATE}", start_state)
                .replace("{END_STATE}", end_state),
            desc: q.desc.clone()
        }
    }

}

pub fn load_queries_from_json(queries: &str) -> Result<HashMap<String, CQuery>, Box<dyn Error>> {
    // let mut file = File::open(path)?;
    // let mut content = String::new();
    // file.read_to_string(&mut content)?;

    let json: Value = serde_json::from_str(queries)?;

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
