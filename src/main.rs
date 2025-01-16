// src/main.rs
use actix_web::{get, http, web, App, post, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger};
use serde::{Deserialize, Serialize};
use std::io;

// Import your grammar calculation logic
mod grammar;
mod calculator;
mod mg;
use dotenv::dotenv;
use std::env;
use calculator::Calculate;
use mg::GrammarGraph;
use mg::MGParser;


// Define a struct for the input
#[derive(Deserialize)]
struct GrammarInput {
    grammar: String,
}

// Define a struct for the output
#[derive(Serialize)]
struct GrammarSizeResponse {
    size: f64,
}

#[get("/health")]
async fn health_check() -> impl Responder {
    "Service is up and running!"
}

#[post("/calculate")]
async fn calculate_size(input: web::Json<GrammarInput>) -> HttpResponse {
    let grammar = match grammar::Grammar::new(&input.grammar, 26, 7, ';') {
        Ok(g) => g, // If successful, bind the grammar to `g`
        Err(e) => panic!("Failed to create Grammar: {}", e), 
    };

    let calculator: calculator::GrammarSizeCalculator = calculator::GrammarSizeCalculator;
    let size: f64 = calculator.get_grammar_size(&grammar, false);
    let response = GrammarSizeResponse { size };

    // shove some code in here to see if it works
    let mut mg_parser: MGParser = MGParser::new();
    mg_parser.parse_grammar_representation(&input.grammar);
    mg_parser.to_json();

    dotenv().ok();
    let secret_key = env::var("PASSWORD")
        .expect("The database password must be set in a .env file.");

    let grammar_graph = match GrammarGraph::new(
        "neo4j://localhost:7687",
        "neo4j",
        secret_key.as_str(),
    ).await {
        Ok(g) => g,
        Err(e) => panic!("{}", e),
    };
    // create_example(grammar_graph).await?;
    let _ = Some(grammar_graph.clear().await);

    let result = mg_parser.create_grammar_graph(&grammar_graph).await;
    match result {
        Ok(_) => {
            println!("Query executed successfully.");
        }
        Err(e) => {
            // Handle the Neo4j error here
            eprintln!("Error running the query: {}", e);
        }
    }
    println!("Updated the grammar graph.");

    // Return the size as a JSON response
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    // .allowed_origin("http://localhost:8080") // accepts this origin
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .service(calculate_size)
            .service(health_check)
    })
    .bind(("127.0.0.1", 8000))? // the actual route that it is hosted on
    .workers(2)
    .run()
    .await
}