mod mg; // Import the module

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use crate::mg::*;

    // Create a general graph
    let general_graph = mg::GeneralGraph::new("neo4j://localhost:7687", "neo4j", "B1R2C3K4h5m6p7t8n9$$").await?;
    general_graph.create_node("Person", "{name: 'Alice', age: 30}").await?;

    // Create a grammar graph
    let grammar_graph = mg::GrammarGraph::new(
        "neo4j://localhost:7687",
        "neo4j",
        "B1R2C3K4h5m6p7t8n9$$",
        "My Grammar Graph",
    )
    .await?;
    grammar_graph.create_state("InitialState").await?;

    Ok(())
}