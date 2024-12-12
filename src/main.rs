mod mg; 
use dotenv::dotenv;
use mg::mg::GrammarGraph;
use mg::mg::Edge;
use mg::mg::MGParser;
use mg::mg::LexicalItem;
use mg::mg::LIRelation;
use core::panic;
use std::env;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use crate::mg::*;

    let mg_parser: MGParser = MGParser::new();
    let rep = mg_parser.parse_grammar_representation("laugh :: =d +k t");
    for li in rep {
        println!("{}", li.morph); 
        for f in li.bundle {
            println!("ID: {}", f.id);
            match f.rel {
                LIRelation::LMerge => println!("LMERGE"),
                LIRelation::RMerge => println!("RMERGE"),
                LIRelation::PlusMove => println!("+ Move"),
                LIRelation::MinusMove => println!("- Move"),
                LIRelation::State => println!("State")
            }
        }
    }
    panic!("Finishing...");

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
    gg.connect_states("t", "n", "superdyduper").await?;
    gg.connect_states("n", "t", "lol").await?;
    gg.connect_states("n", "d", "this").await?;
    gg.connect_states("d", "v", "laugh").await?;
    gg.connect_states("v", "g", "`-ing`").await?;
    gg.connect_states("v", "t", "`-s`").await?;
    gg.connect_states("g", "t", "is").await?;
    let edge: Edge<'_> = Edge {
        state_a_id: "n",
        state_b_id: "d",
        rel: "this"
    };
    // gg.delete_edge(&edge).await?;
    gg.contract_edge(&edge).await?;
    Ok(())
}