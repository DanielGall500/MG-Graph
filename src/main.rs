mod mg; 
use dotenv::dotenv;
use std::env;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use crate::mg::*;

    dotenv().ok();
    let secret_key = env::var("PASSWORD")
        .expect("The database password must be set in a .env file.");

    let grammar_graph = mg::GrammarGraph::new(
        "neo4j://localhost:7687",
        "neo4j",
        secret_key.as_str(),
        "My Grammar Graph",
    )
    .await?;
    grammar_graph.clear().await?;
    grammar_graph.create_state("Sam").await?;
    grammar_graph.create_state("Daniel").await?;
    grammar_graph.connect_states("Sam", "Daniel", "omgtheybeinarelationship").await?;
    grammar_graph.connect_states("Daniel", "Sam", "woaaahrelationship").await?;

    Ok(())
}