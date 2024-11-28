pub mod mg {
    use neo4rs::{query, Graph};
    use std::error::Error;

    pub struct GeneralGraph {
        graph: Graph,
    }

    impl GeneralGraph {
        pub async fn new(db_id: &str, username: &str, password: &str) -> Result<Self, Box<dyn Error>> {
            let graph = Graph::new(db_id, username, password).await?;
            println!("Connected to database.");
            Ok(Self{ graph })
        }

        pub async fn create_node(&self, label: &str, properties: &str) -> Result<(), Box<dyn Error>> {
            // Dynamically construct the query string with label and properties
            // let query = format!("CREATE (p:{} {})", label, properties);
            
            // Execute the query
            self.graph.run(query("CREATE (p: State { name: $name })").param("name", "Jack")).await?;
            Ok(())
        }
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
            let properties = format!("{{name: '{}'}}", name);
            self.base.create_node("State", &properties).await?;
            Ok(())
        }

        pub fn get_title(&self) -> &str {
            &self.graph_title
        }
    }
}
