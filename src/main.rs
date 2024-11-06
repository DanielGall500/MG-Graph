// src/main.rs
use actix_web::{get, http, web, App, post, HttpRequest, HttpResponse, HttpServer};
use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger};
use serde::{Deserialize, Serialize};
use std::io;

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

#[get("/index.html")]
async fn index(req: HttpRequest) -> &'static str {
    "<p>Hello World!</p>"
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:8080")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .service(calculate_size)
    })
    .bind(("127.0.0.1", 8000))?
    .workers(2)
    .run()
    .await
}