// src/main.rs
use actix_web::{get, web, App, post, 
    HttpResponse, HttpServer, Responder, 
    http::header, middleware::Logger};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use core::panic;
use std::{io, env};
use std::sync::Mutex;
use std::error::Error;
use dotenv::dotenv;
use std::collections::HashMap;

mod calculator;
mod cypher;
mod parse;

use calculator::Calculate;
use parse::{
    mg::{GrammarGraph, MGParser, LexicalItem},
    grammar::Grammar,
    decomp::{Decomposer,Affix},
};

#[derive(Deserialize)]
struct GrammarInput {
    grammar: String,
}

#[derive(Serialize)]
struct GrammarSizeResponse {
    size: f64,
}

#[get("/health")]
async fn health_check() -> impl Responder {
    "Service is up and running!"
}

struct MGState {
    mg: Mutex<Vec<LexicalItem>>,
    mg_parser: Mutex<MGParser>,
    graph: Mutex<GrammarGraph>,
    decomposer: Mutex<Decomposer>
}

async fn update_mg(data: &web::Data<MGState>, updated: Vec<LexicalItem>) -> Result<(), Box<dyn Error>> {
    let mut mg_state = data.mg.lock().unwrap();
    let mut mg_parser = data.mg_parser.lock().unwrap();
    let mut graph = data.graph.lock().unwrap();

    *mg_state = updated;
    graph.clear().await?;
    mg_parser.create_grammar_graph(&graph).await?;
    Ok(())
}


#[derive(Deserialize)]
struct DecomposeInput {
    affix: String,
    split: usize,
    lis_to_decompose: Vec<usize>
}
#[post("/decompose")]
async fn decompose(data: web::Data<MGState>, input: web::Json<DecomposeInput>) -> HttpResponse {
    /* We have a function which decomposes the MG, now we need to handle the input. */
    let mg_state = data.mg.lock().unwrap();
    let decomposer = data.decomposer.lock().unwrap();
    let affix: Affix = Affix {
        morph: input.affix.to_string(),
    };
    let split: usize = input.split;
    let to_decomp = &input.lis_to_decompose;

    let decomposed_mg: Vec<LexicalItem>;
    match decomposer.decompose(mg_state.to_vec(), to_decomp.clone(), affix, split) {
        Ok(decomp) => decomposed_mg = decomp,
        Err(e) => {
            eprintln!("DECOMP ERROR - Could Not Perform Decomposition: {}", e);
            decomposed_mg = mg_state.clone();
        } 
    }
    match update_mg(&data, decomposed_mg).await {
        Ok(()) => println!("MG Updated"),
        Err(e) => eprintln!("{}", e)
    }
    HttpResponse::Ok().into()
}

#[derive(Serialize, Deserialize)]
struct DecomposeSuggestionResponse {
    prefix_morph_map: HashMap<String, Vec<String>>
}
#[get("/decompose-suggestions")]
async fn get_decompose_suggestions(data: web::Data<MGState>) -> HttpResponse {
    let mut mg_state = data.mg.lock().unwrap();

    let mut mg_parser = data.mg_parser.lock().unwrap();
    let mut graph = data.graph.lock().unwrap();
    let mut decomposer = data.decomposer.lock().unwrap();

    let suggestions = decomposer.get_decompose_suggestions(&mg_state);

    let suggestions_prefix_morph_map = suggestions
                    .iter()
                    .map(|(x, y)| {
                        let morph_vec = y.iter()
                            .map(|index| mg_state.get(index.clone()).unwrap().clone().morph)
                            .collect::<Vec<_>>(); // Collect morphs into a Vec<String>
                        (x.clone(), morph_vec) // Form (key, value) pair
                    })
                    .collect(); 

    let response = DecomposeSuggestionResponse {
        prefix_morph_map: suggestions_prefix_morph_map,
    };

    HttpResponse::Ok().json(response)
}

#[post("/calculate")]
async fn calculate_size(data: web::Data<MGState>, input: web::Json<GrammarInput>) -> HttpResponse {
    let grammar = match Grammar::new(&input.grammar, 26, 7, ';') {
        Ok(g) => g, // If successful, bind the grammar to `g`
        Err(e) => panic!("Failed to create Grammar: {}", e), 
    };

    let calculator: calculator::GrammarSizeCalculator = calculator::GrammarSizeCalculator;
    let size: f64 = calculator.get_grammar_size(&grammar, false);

    let mut mg_parser = data.mg_parser.lock().unwrap();

    // shove some code in here to see if it works
    mg_parser.parse_grammar_representation(&input.grammar);
    let mg_updated = mg_parser.get_grammar().clone();
    mg_parser.to_json("recent");

    update_mg(&data, mg_updated);

    let response = GrammarSizeResponse { size };
    HttpResponse::Ok().json(response)
}

 /* 
fn main() {
    let mut mg_parser: MGParser = MGParser::new();
    let mg: &str = "Mary :: d -k;
                laughs :: =d +k t;
                laughed :: =d +k t;
                jumps :: =d +k t;
                jumped :: =d +k t;"; 
    let result: &Vec<LexicalItem>;

    println!("MG");
    println!("{}", mg);

    match mg_parser.parse_grammar_representation(mg) {
        Ok(grammar) => {
            parse::decomp::test_decompose_affix_finder(grammar);
        }
        Err(e) => panic!("Error while testing.")
    }

    match mg_parser.to_json("original") {
        Ok(()) => println!("Converted to JSON."),
        Err(e) => eprintln!("Invalid JSON")
    }
    
}
*/

const GRAPH_DATABASE_ADDR: &str = "neo4j://localhost:7687";
const GRAPH_DATABASE_USER: &str = "neo4j";

#[actix_web::main]
async fn main() -> io::Result<()> {

    /* a neo4j instance password must be set in an env file */
    dotenv().ok();
    let secret_key = env::var("PASSWORD")
        .expect("The database password must be set in a .env file.");

    /* connect to the neo4j instance */
    let grammar_graph = match GrammarGraph::new(
        GRAPH_DATABASE_ADDR,
        GRAPH_DATABASE_USER,
        secret_key.as_str(),
    ).await {
        Ok(g) => g,
        Err(e) => panic!("NEO4J ERROR: {}", e),
    };

    let mg_state = web::Data::new(
        MGState {
        mg: Mutex::new(Vec::new()),
        mg_parser: Mutex::new(MGParser::new()),
        graph: Mutex::new(grammar_graph),
        decomposer: Mutex::new(Decomposer::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(mg_state.clone())
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