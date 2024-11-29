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

        pub async fn create_node(&self, category: &str, label_id: &str, label_val: &str) -> Result<(), Box<dyn Error>> {
            let create_node_query = format!("CREATE (p:{} {{ {}: \"{}\" }})", category, label_id, label_val);
            self.graph.run(query(create_node_query.as_str())).await?;
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

        pub async fn set_relationship(&self, cat_a: &str, cat_b: &str, node_a: &str, node_b: &str, rel: &str) -> Result<(), Box<dyn Error>> {
            let set_relationship_query = format!(
                "MATCH (a:{} {{ name: \"{}\" }}), (b:{} {{name: \"{}\" }})
                CREATE (a)-[:{}]->(b)
                RETURN a, b", cat_a, node_a, cat_b, node_b, rel);
            self.graph.run(query(set_relationship_query.as_str())).await?;
            Ok(())
        }

        pub async fn clear(&self) -> Result<(), Box<dyn Error>> {
            let remove_all_query: &str = "MATCH (n) DETACH DELETE n";
            self.graph.run(query(remove_all_query)).await?;
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
            // let properties = format!("{{name: '{}'}}", name);
            self.base.create_node("State", "name", name).await?;
            Ok(())
        }

        pub async fn connect_states(&self, state_a: &str, state_b: &str, rel: &str) -> Result<(), Box<dyn Error>> {
            self.base.set_relationship("State", "State", state_a, state_b, rel).await?;
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
}
