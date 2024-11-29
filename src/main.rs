mod mg; 
use dotenv::dotenv;
use mg::mg::GrammarGraph;
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
        "Minimalist Grammar",
    )
    .await?;
    create_example(grammar_graph).await?;

    Ok(())
}

async fn create_example(gg: GrammarGraph) -> Result<(), Box<dyn std::error::Error>> {
    gg.clear().await?;
    gg.create_state("n").await?;
    gg.create_state("d").await?;
    gg.create_state("v").await?;
    gg.create_state("g").await?;
    gg.create_state("t").await?;
    gg.connect_states("n", "d", "this").await?;
    gg.connect_states("d", "v", "laugh").await?;
    gg.connect_states("v", "g", "`-ing`").await?;
    gg.connect_states("v", "t", "`-s`").await?;
    gg.connect_states("g", "t", "is").await?;
    Ok(())
}