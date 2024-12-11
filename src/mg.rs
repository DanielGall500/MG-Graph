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
    
    pub struct LexicalItem {
        morph: String,
        bundle: Vec<String>,
    }

    pub struct MGParser {
        states: Vec<String>,
    }

    impl MGParser {
        pub fn new() -> Self {
            Self{
                states: Vec::new(),
            }
        }

        pub fn parse_grammar_representation(&self, minimalist_grammar: &str) -> Result<(), Box<dyn Error>> {
            let mut lexical_items: Vec<LexicalItem> = Vec::new();
            let mut lines = minimalist_grammar.lines();
            for l in lines.into_iter() {

                let mut li: LexicalItem = LexicalItem { morph: String::from(""), bundle: Vec::new() };

                // e.g laughs :: d= +k t
                let morph_feature_split: Vec<String> = l.split("::").map(|c| c.to_string()).collect();

                if let Some(morph) = morph_feature_split.get(0) {
                    li.morph = morph.trim().to_string();
                    println!("Morph: {}", li);
                }
                if let Some(features) = morph_feature_split.get(1) {
                    let individual_feature_split: Vec<String> = features.split_whitespace().map(|c| c.trim().to_string()).collect();
                    for feature in individual_feature_split {
                        li.bundle.push(feature);
                        println!("-{}-", feature);
                    }
                }
                lexical_items.push(li);
            }
            Ok(())
        }
    }
}
