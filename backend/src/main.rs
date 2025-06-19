use actix_web::body;
// src/main.rs
use actix_web::{get, web, App, post, 
    HttpResponse, HttpServer, Responder, 
    http::header, middleware::Logger};
use actix_cors::Cors;
use parse::parser::Parser;
use serde::{Deserialize, Serialize};
use core::panic;
use std::{io};
use std::error::Error;
use std::collections::HashMap;

use tokio::sync::{Mutex, RwLock};

mod calculator;
mod cypher;
mod parse;
mod data;

use calculator::{Calculate, MDL};
use parse::{
    graph::GrammarGraph,
    mg::{MG, LexicalItem},
    grammar::Grammar,
    decomp::{Decomposer,Affix},
};
use data::storage::{DataManager, MGCollection, MGExample, Settings};

#[get("/health")]
async fn health_check() -> impl Responder {
    "Service is up and running!"
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

        match mg_parser.to_json("recent").await {
            Ok(()) => println!("Updated JSON with decomposition step."),
            Err(e) => eprintln!("{}",e),
        }
    }

}

async fn parse_new_mg(data: &web::Data<MGState>, grammar: &str) -> Result<Vec<LexicalItem>, Box<dyn Error>> {
    println!("Parsing New MG");
    let mut mg_parser = data.mg_parser.lock().await;

    match Parser::convert_text_to_stored(grammar, &mut mg_parser) {
        Ok(()) => {
            println!("Successful grammar parsing.");
        }
        Err(e) => println!("Invalid grammar parse: {}", e),
    }

    match mg_parser.to_json("recent").await {
        Ok(()) => println!("Successful JSON conversion for new MG parsing."),
        Err(e) => println!("Invalid JSON conversion: {}", e),
    }
    Ok(mg_parser.get_grammar().clone())
}

/* Improve safety here. No return. */
async fn update_grammar_graph(data: &web::Data<MGState>) {
    println!("Updating Grammar Graph");
    let graph_guard = data.graph_db.read().await;

    if let Some(db) = graph_guard.as_ref() {
        match db.clear().await {
            Ok(()) => println!("Graph cleared."),
            Err(e) => println!("ERROR: Unable to clear graph. {}", e)
        }

        // below brackets for sync code only?
        {
            let mut mg_parser = data.mg_parser.lock().await;
            let lis = mg_parser.get_grammar();

            println!("LIs in grammar to be passed to graph:");
            for li in lis.iter() {
                println!("{}", li.morph);
            }


            match Parser::convert_stored_to_graph(&mut mg_parser, &*db).await {
                Ok(_g) => println!("Graph updated successfully."),
                Err(e) => println!("Problem updating graph: {}", e)
            }

            if let Err(e) = db.remove_redundancy().await {
                println!("Couldn't remove redundancy.");
            }
    
        }
    }
}


fn calculate_size_from_string(grammar: &str, alphabet_size: usize, num_types: usize) -> MDL {
    let grammar = match Grammar::new(grammar, alphabet_size, num_types, ';') {
        Ok(g) => g, // If successful, bind the grammar to `g`
        Err(e) => panic!("Failed to create Grammar: {}", e), 
    };

    let calculator: calculator::GrammarSizeCalculator = calculator::GrammarSizeCalculator;
    let size: MDL = calculator.get_grammar_size(&grammar, false);
    size
}


#[derive(Serialize)]
struct GrammarSizeResponse {
    grammar: String,
    size: MDL,
}
#[derive(Deserialize)]
struct CalculateSizeInput {
    alphabet_size: usize,
    num_types: usize
}
#[post("/calculate-size")]
async fn request_calculate_size(data: web::Data<MGState>, input: web::Json<CalculateSizeInput>) -> HttpResponse {
    // calculate the size of the MG
    // converts to a text representation first
    let mg_parser = data.mg_parser.lock().await;
    let mg_as_str = format!("{}", mg_parser);
    let size: MDL = calculate_size_from_string(&mg_as_str, input.alphabet_size.clone(), input.num_types.clone());

    let response = GrammarSizeResponse { grammar: mg_as_str, size };
    HttpResponse::Ok().json(response)
}

#[derive(Deserialize)]
struct GrammarInput {
    grammar: String,
    alphabet_size: usize,
    num_types: usize
}
#[post("/build-initial-mg")]
async fn build_initial_mg(data: web::Data<MGState>, input: web::Json<GrammarInput>) -> HttpResponse {
    let new_mg = parse_new_mg(&data, &input.grammar).await;
    update_grammar_graph(&data).await;
    update_mg(&data, new_mg.unwrap()).await;

    let size: MDL = calculate_size_from_string(&input.grammar, input.alphabet_size.clone(), input.num_types.clone());
    let response = GrammarSizeResponse { grammar: input.grammar.clone(), size };
    HttpResponse::Ok().json(response)
}

#[derive(Deserialize)]
struct CombinationInput {
    state_a: String,
    state_b: String,
}
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}
#[post("/combine")]
async fn combine(
    data: web::Data<MGState>,
    _input: web::Json<CombinationInput>,
) -> HttpResponse {
    println!("--> combine handler called");

    let graph_guard = data.graph_db.read().await;
    println!("--> graph lock acquired");

    if let Some(graph) = graph_guard.as_ref() {
        println!("--> graph is Some");

        if let Err(e) = graph.contract_edge(
            &_input.state_a,
            &_input.state_b,
        ).await {
            eprintln!("!!! contract_edge failed: {:?}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("Contracting failed: {}", e),
            });
        }

        println!("--> contract_edge succeeded");
        return HttpResponse::Ok().json(serde_json::json!({
            "message": "contracted successfully"
        }));
    } else {
        println!("!!! graph is None");
        return HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Graph not initialized".into(),
        });
    }
}

/* 
#[post("/combine")]
async fn combine(data: web::Data<MGState>, _input: web::Json<CombinationInput>) -> HttpResponse {
    let graph_guard = data.graph_db.read().await;

    if let Some(graph) = graph_guard.as_ref() {
        if let Err(e) = graph.contract_edge(
            &_input.state_a, 
            &_input.state_b, 
            &_input._li).await {
            eprintln!("Error: {}", e);
            let err_response = ErrorResponse {
                error: format!("Contracting failed: {}", e),
            };
            return HttpResponse::InternalServerError().json(err_response);
        }
    }
    
    HttpResponse::Ok().finish()
}
    */


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
    let mg_state = data.mg.lock().await;

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

#[derive(Serialize, Deserialize)]
struct PathwayResponse {
    all_pathways: Vec<String>,
    shortest_pathways: Vec<String>,
}
#[derive(Serialize, Deserialize)]
struct PathwayInput {
    start_item: String,
    end_item: String
}
#[post("/pathways")]
async fn pathways(data: web::Data<MGState>, input: web::Json<PathwayInput>) -> HttpResponse {
    let graph_guard = data.graph_db.read().await;

    if let Some(graph) = graph_guard.as_ref() {
        let poss_paths = graph.get_possible_paths(&input.start_item, &input.end_item).await.unwrap();
        let shortest_paths = graph.get_shortest_paths(&input.start_item, &input.end_item).await.unwrap();
        let response: PathwayResponse = PathwayResponse {
            all_pathways: poss_paths,
            shortest_pathways: shortest_paths,
        };
        HttpResponse::Ok().json(response)
    }
    else {
        let response: PathwayResponse = PathwayResponse {
            all_pathways: Vec::new(),
            shortest_pathways: Vec::new(),
        };
        HttpResponse::InternalServerError().json(response)
    }

}

#[derive(Serialize, Deserialize)]
struct SaveMGInput {
    title: String,
    lang: String,
    grammar: Vec<String>
}

#[post("/store-mg")]
async fn store_mg(input: web::Json<MGExample>) -> impl Responder {

    let mut my_mgs: MGCollection;
    match DataManager::load_mg_collection::<MGCollection>().await {
        Ok(data) => my_mgs = data,
        Err(_e) => {
            my_mgs = MGCollection::new(); 
        }
    }

    my_mgs.push(input.into_inner());

    println!("Saving text...");
    if let Err(e) = DataManager::save_mg_collection(&my_mgs).await {
        eprintln!("Failed to save text: {}", e);
        return HttpResponse::InternalServerError().body("Failed to write to file");
    }
    println!("Text saved.");
    HttpResponse::Ok().body("Grammar stored.")
}

#[derive(Serialize, Deserialize)]
struct GrammarIndex {
    index: usize
}
#[post("/delete-mg")]
async fn delete_mg(input: web::Json<GrammarIndex>) -> impl Responder {

    let mut my_mgs: MGCollection;
    match DataManager::load_mg_collection::<MGCollection>().await {
        Ok(data) => my_mgs = data,
        Err(_e) => {
            return HttpResponse::InternalServerError().body("Failed to delete MG. Couldn't be loaded.");
        }
    }

    my_mgs.remove(input.index);

    println!("Saving text...");
    if let Err(e) = DataManager::save_mg_collection(&my_mgs).await {
        eprintln!("Failed to save text: {}", e);
        return HttpResponse::InternalServerError().body("Failed to write to file");
    }
    println!("Text saved.");
    HttpResponse::Ok().body("Grammar stored.")

}

#[derive(Serialize, Deserialize)]
struct DBAuth {
    db_addr: String,
    db_name: String,
    username: String,
    password: String,
}
#[post("/store-db-auth")]
async fn store_db_auth(db_auth: web::Json<DBAuth>) -> impl Responder {

    if let Err(e) = DataManager::save_settings(&db_auth).await {
        eprintln!("Failed to save text: {}", e);
        return HttpResponse::InternalServerError().body(format!("Database authentication details unable to be stored. {}", e));
    }

    HttpResponse::Ok().body("Database authentication details stored.")
}

#[get("/load-mg-collection")]
async fn load_mg_collection() -> impl Responder {
    match DataManager::load_mg_collection::<MGCollection>().await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => {
            eprintln!("Failed to load JSON: {}", e);
            HttpResponse::InternalServerError().body("Failed to read or parse file")
        }
    }
}

#[get("/test-db-auth")]
async fn test_db_auth(data: web::Data<MGState>) -> impl Responder {
    let settings: Settings = match DataManager::load_settings::<Settings>().await {
        Ok(data) => {
            data
        }
        Err(e) => {
            eprintln!("Unable to load settings: {}", e);
            return HttpResponse::InternalServerError().body(format!("Unable to access settings. {}", e));
        }
    };

    if let Err(e) = connect_to_neo4j(data, settings.db_addr.as_str(), 
    settings.db_name.as_str(), 
    settings.username.as_str(), 
    settings.password.as_str()).await {
            eprintln!("Unable to establish a connection: {}", e);
            return HttpResponse::InternalServerError().body("Unable to establish connection.");
    }
    HttpResponse::Ok().body("Connected")
}

async fn load_settings() -> Result<Settings, Box<dyn Error>> {
    let settings =  DataManager::load_settings::<Settings>().await?;
    Ok(settings)
}

#[get("/get-settings")]
async fn get_settings() -> impl Responder {
    match load_settings().await {
        Ok(settings) => {
            HttpResponse::Ok().json(settings)
        }
        Err(e) => {
            eprintln!("Settings could not be loaded: {}", e);
            HttpResponse::InternalServerError().body(format!("Settings could not be loaded. {}", e))
        }
    }
}

async fn connect_to_neo4j(data: web::Data<MGState>, db_addr: &str, db_name: &str, db_username: &str, db_pw: &str) -> Result<(), Box<dyn Error>> {
    let mut guard = data.graph_db.write().await;
    if let Some(db) = guard.as_mut() {
        match db.connect(db_addr, db_name, db_username, db_pw).await {
            Ok(()) => println!("Successfully connected."),
            Err(e) => {
                eprintln!("DB Auth Failed: {}", e);
                return Err(e);
            }
        }
    }
    else {
        let mut new_graph_db = GrammarGraph::new(db_addr, 
            db_name, 
            db_username, 
            db_pw).await?;

        new_graph_db.test_connection().await?;
    }

    Ok(())
}

#[get("/get-mg-json")]
async fn get_mg_json(data: web::Data<MGState>) -> impl Responder {
    let mg_parser = data.mg_parser.lock().await;

    match mg_parser.from_json_raw("recent") {
        Ok(json) => {
            HttpResponse::Ok().body(json)
        }
        Err(_e) => {
            eprintln!("Unable to get current MG as JSON.");
            HttpResponse::InternalServerError().body("Unable to retrieve MG.")
        }
    }

}

const LOCAL_BACKEND_IP: &str = "127.0.0.1";
const LOCAL_BACKEND_PORT: u16 = 8000;

struct MGState {
    mg: Mutex<Vec<LexicalItem>>,
    mg_parser: Mutex<MG>,
    graph_db: RwLock<Option<GrammarGraph>>,
    decomposer: Mutex<Decomposer>
}

#[actix_web::main]
async fn main() -> io::Result<()> {

    let mut grammar_graph: Option<GrammarGraph> = None;
    match load_settings().await {
        Ok(settings) => {
            println!("Settings loaded: {:?}", settings);

            /* connect to the neo4j instance */
            grammar_graph = match GrammarGraph::new(
                &settings.db_addr,
                &settings.db_name,
                &settings.username,
                &settings.password
            ).await {
                Ok(g) => Some(g),
                Err(e) => panic!("NEO4J ERROR: {}", e),
            };
            if let Some(ref g) = grammar_graph {
                match g.clear().await {
                    Ok(()) => println!("Graph cleared."),
                    Err(e) => println!("ERROR: Unable to clear graph. {}", e)
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to load settings: {}", e);
            // Handle the error (retry, fallback, etc.)
        }
    }

    let mg_state = web::Data::new(
        MGState {
        mg: Mutex::new(Vec::new()),
        mg_parser: Mutex::new(MG::new()),
        graph_db: RwLock::new(grammar_graph),
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
            .service(pathways)
            .service(store_mg)
            .service(delete_mg)
            .service(load_mg_collection)
            .service(store_db_auth)
            .service(test_db_auth)
            .service(get_settings)
            .service(get_mg_json)
    })
    .bind((LOCAL_BACKEND_IP, LOCAL_BACKEND_PORT))? // the actual route that it is hosted on
    .workers(2)
    .run()
    .await
}
