// src/main.rs
use actix_web::{post, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

// Import your grammar calculation logic
mod grammar;
mod calculator;
use calculator::Calculate;

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

#[post("/calculate")]
async fn calculate_size(input: web::Json<GrammarInput>) -> HttpResponse {
    // Calculate the grammar size using your function
    let grammar = match grammar::Grammar::new(&input.grammar, 26, 7, ';') {
        Ok(g) => g, // If successful, bind the grammar to `g`
        Err(e) => panic!("Failed to create Grammar: {}", e), 
    };

    let calculator: calculator::GrammarSizeCalculator = calculator::GrammarSizeCalculator;
    let size: f64 = calculator.get_grammar_size(&grammar, false);
    let response = GrammarSizeResponse { size };

    // Return the size as a JSON response
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(calculate_size) // Register the calculate_size endpoint
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
