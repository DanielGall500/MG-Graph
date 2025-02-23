// src/main.rs
use actix_web::{get, web, App, post, 
    HttpResponse, HttpServer, Responder, 
    http::header, middleware::Logger};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use core::panic;
use tokio::sync::Mutex;
use std::{io, env};
use std::sync::Arc;
use std::error::Error;
use std::collections::HashMap;
use dotenv::dotenv;

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
    grammar: String,
    size: f64,
}

#[get("/health")]
async fn health_check() -> impl Responder {
    "Service is up and running!"
}

struct MGState {
    mg: Mutex<Vec<LexicalItem>>,
    mg_parser: Mutex<MGParser>,
    graph_db: Arc<GrammarGraph>,
    decomposer: Mutex<Decomposer>
}

async fn update_mg(data: &web::Data<MGState>, updated: Vec<LexicalItem>) {

    println!("Updating MG");
    {
        println!("Updating 1");
        let mut mg_state = data.mg.lock().await;
        println!("Updating 2");
        *mg_state = updated.clone();
        println!("Updating 3");
    }

    println!("Second run.");
    {
        let mut mg_parser = data.mg_parser.lock().await;
        mg_parser.update_grammar(updated);

        match mg_parser.to_json("recent") {
            Ok(()) => println!("Updated JSON with decomposition step."),
            Err(e) => eprintln!("{}",e),
        }
    }

}

async fn parse_new_mg(data: &web::Data<MGState>, grammar: &String) -> Result<Vec<LexicalItem>, Box<dyn Error>> {
    println!("Parsing New MG");
    let mut mg_parser = data.mg_parser.lock().await;
    let grammar_rep = mg_parser.parse_grammar_representation(grammar);
    match grammar_rep {
        Ok(mg) => {
            println!("Successful grammar parsing.");
        }
        Err(e) => println!("Invalid grammar parse: {}", e),
    }

    match mg_parser.to_json("recent") {
        Ok(()) => println!("Successful JSON conversion for new MG parsing."),
        Err(e) => println!("Invalid JSON conversion: {}", e),
    }
    Ok(mg_parser.get_grammar().clone())
}

async fn update_grammar_graph(data: &web::Data<MGState>) {
    println!("Updating Grammar Graph");
    let db = &data.graph_db;

    match db.clear().await {
        Ok(()) => println!("Graph cleared."),
        Err(e) => println!("ERROR: Unable to clear graph. {}", e)
    }

    {
        let mut mg_parser = data.mg_parser.lock().await;
        let lis = mg_parser.get_grammar();

        println!("LIs in grammar to be passed to graph:");
        for li in lis.iter() {
            println!("{}", li.morph);
        }


        match mg_parser.create_grammar_graph(db).await {
            Ok(g) => println!("Graph updated successfully."),
            Err(e) => println!("Problem updating graph: {}", e)
        }
    
    }
}

fn calculate_size_from_string(grammar: &str) -> f64 {
    let grammar = match Grammar::new(grammar, 26, 7, ';') {
        Ok(g) => g, // If successful, bind the grammar to `g`
        Err(e) => panic!("Failed to create Grammar: {}", e), 
    };

    let calculator: calculator::GrammarSizeCalculator = calculator::GrammarSizeCalculator;
    let size: f64 = calculator.get_grammar_size(&grammar, false);
    size
}

#[get("/calculate-size")]
async fn request_calculate_size(data: web::Data<MGState>) -> HttpResponse {
    println!("BEGINNING SIZE CALCULATION");
    // calculate the size of the MG
    // converts to a text representation first
    let mg_parser = data.mg_parser.lock().await;
    let mg_as_str = format!("{}", mg_parser);
    let size: f64 = calculate_size_from_string(&mg_as_str);
    println!("NEW SIZE: {}", size);

    let response = GrammarSizeResponse { grammar: mg_as_str, size };
    HttpResponse::Ok().json(response)
}

#[post("/build-initial-mg")]
async fn build_initial_mg(data: web::Data<MGState>, input: web::Json<GrammarInput>) -> HttpResponse {
    let new_mg = parse_new_mg(&data, &input.grammar).await;
    update_grammar_graph(&data).await;
    update_mg(&data, new_mg.unwrap()).await;

    let size: f64 = calculate_size_from_string(&input.grammar);
    println!("Size of MG: {}", size);
    let response = GrammarSizeResponse { grammar: input.grammar.clone(), size };
    HttpResponse::Ok().json(response)
}

#[derive(Deserialize)]
struct CombinationInput {
    state_a: String,
    state_b: String,
}
#[post("/combine")]
async fn combine(data: web::Data<MGState>, input: web::Json<CombinationInput>) -> HttpResponse {
    let mut mg_parser = data.mg_parser.lock().await;
    /*
    Should combine two states.
     */
    HttpResponse::Ok().into()
}


/*
NOTE:
- Doesn't yet work for multiple features. Connections don't connect properly.
 */
#[derive(Serialize, Deserialize)]
struct DecomposeInput {
    affix: String,
    split: usize,
}

#[post("/decompose")]
async fn decompose(data: web::Data<MGState>, input: web::Json<DecomposeInput>) -> HttpResponse {
    let decomposed_mg: Vec<LexicalItem>;
    // initial state access
    {
        /* We have a function which decomposes the MG, now we need to handle the input. */
        let mg_state = data.mg.lock().await;
        let decomposer = data.decomposer.lock().await;

        let candidate_map = decomposer.candidate_map.clone();
        let to_decomp = candidate_map.get(&input.affix.to_string()).unwrap();
        // let to_decomp = &input.lis_to_decompose;

        let affix: Affix = Affix {
            morph: input.affix.to_string(),
        };
        println!("Affix: {:?}", affix.morph);
        let split: usize = input.split;

        match decomposer.decompose(mg_state.to_vec(), to_decomp.clone(), affix, split) {
            Ok(decomp) => {
                decomposed_mg = decomp;
                println!("No Error. Decomp run.");
            },
            Err(e) => {
                eprintln!("DECOMP ERROR - Could Not Perform Decomposition: {}", e);
                decomposed_mg = mg_state.clone();
            } 
        }

    }

    {
        update_mg(&data, decomposed_mg).await;
    }

    // second state access
    {
        update_grammar_graph(&data).await;
    }
    HttpResponse::Ok().into()
}

#[derive(Serialize, Deserialize)]
struct DecomposeSuggestionResponse {
    prefix_morph_map: HashMap<String, Vec<String>>,
    test: String,
}
#[get("/decompose-suggestions")]
async fn get_decompose_suggestions(data: web::Data<MGState>) -> HttpResponse {
    let mut mg_state = data.mg.lock().await;

    let mut mg_parser = data.mg_parser.lock().await;
    let graph = &data.graph_db;
    // let mut graph = data.graph.lock().await;
    let mut decomposer = data.decomposer.lock().await;

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
        test: String::from("Working!")
    };

    HttpResponse::Ok().json(response)
}

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

    // empty the database on each reload
    match grammar_graph.clear().await {
        Ok(()) => println!("Graph cleared."),
        Err(e) => println!("ERROR: Unable to clear graph. {}", e)
    }

    let mg_state = web::Data::new(
        MGState {
        mg: Mutex::new(Vec::new()),
        mg_parser: Mutex::new(MGParser::new()),
        graph_db: Arc::new(grammar_graph),
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
            .service(request_calculate_size)
            .service(health_check)
            .service(decompose)
            .service(get_decompose_suggestions)
            .service(build_initial_mg)
            .service(combine)
    })
    .bind(("127.0.0.1", 8000))? // the actual route that it is hosted on
    .workers(2)
    .run()
    .await
}