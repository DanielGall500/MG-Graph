use serde_json::Value;
use core::panic;
use std::fs::File;
use std::error::Error;
use std::io::Read;
use std::collections::HashMap;
pub struct CQuery {
    pub name: String,
    pub query: String,
    pub desc: String,
}

pub struct CQueryStorage {
    pub queries: HashMap<String, CQuery>
}

const QUERIES_PATH: &str = "queries.json";

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

    pub fn get_clear_graph(&self) -> &CQuery {
        const Q_ID: &str = "clear_graph";
        for (id,q) in self.queries.iter() {
            eprintln!("Printing Q's: {}", id);
        }
        self.queries.get(Q_ID).expect(&format!("No Query Available For ID {}", Q_ID))
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
