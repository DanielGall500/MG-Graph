use super::graph::GrammarGraph;
use super::mg::MG;
use crate::parse::mg::LIRelation;
use crate::parse::mg::Feature;
use crate::parse::mg::LexicalItem;
use std::error::Error;

pub struct Parser {}

impl Parser {
    pub fn convert_text_to_stored(minimalist_grammar: &str, mg_stored: &mut MG) -> Result<(), Box<dyn Error>> {
        mg_stored.mg.clear();
        mg_stored.states.clear();
        let mut li: LexicalItem;

        let mg_statements = minimalist_grammar
            .split(";")
            .into_iter()
            .filter(|l| { !l.is_empty() });

        for l in mg_statements {
            li = LexicalItem { 
                morph: String::from(""), 
                bundle: Vec::new() 
            };

            // e.g laughs :: d= +k t
            let morph_feature_split: Vec<String> = l.split("::").map(|c| c.to_string()).collect();

            // STEP 1: process the phonological form: e.g "Mary" in "Mary" :: d -k 
            if let Some(morph) = morph_feature_split.get(0) {
                li.morph = morph.trim().to_string();
                println!("Valid Morph: {}", li.morph);
            }
            else {
                eprintln!("Invalid MG Statement: {}", l);
                eprintln!("Error was found in phonological form parsing.")
            }

            // STEP 2: parse the feature bundle e.g 'd -k' in "Mary" :: d -k
            if let Some(features) = morph_feature_split.get(1) {
                let individual_feature_split = features
                    .split_whitespace()
                    .map(|c| c.trim().to_string());

                // STEP 3: iterate over each feature in the LI and add
                // to the feature bundle
                for feature in individual_feature_split {
                    let (relation, id) = 
                    if feature.starts_with("=>") {
                        // need to create new relation for head merge
                        (LIRelation::LMerge, feature[2..].to_string())
                    }
                    else if feature.starts_with("=") {
                        (LIRelation::LMerge, feature[1..].to_string())
                    } else if feature.ends_with("=") {
                        (LIRelation::RMerge, feature[..feature.len() - 1].to_string())
                    } else if feature.starts_with("-") {
                        (LIRelation::MinusMove, feature[1..].to_string())
                    } else if feature.starts_with("+") {
                        (LIRelation::PlusMove, feature[1..].to_string())
                    } else {
                        (LIRelation::State, feature.clone())
                    };

                    // STEP 4: ADD FEATURE INFO TO LI
                    li.bundle.push(Feature {
                        raw: feature.clone(),
                        id: id.clone(),
                        rel: relation,
                    });

                    println!("Valid -{}-", feature.to_string());
                }
            }
            else {
                eprintln!("Invalid MG Statement: {}", l);
                eprint!("Error was found during feature bundle parsing.")
            }
            mg_stored.mg.push(li);
        }
        // Ok(&mg_stored.mg)
        Ok(())
    }

    pub async fn convert_stored_to_graph(mg_stored: &mut MG, mg_graph: &GrammarGraph) -> Result<GrammarGraph, Box<dyn Error>> {
        let mut merge_state: Option<&Feature>;
        let mut final_state: Option<&Feature>;
        let mut move_hoover: Option<&Feature>;
        let mut bundle: &Vec<Feature>;
        let mut is_head: bool;

        mg_stored.states.clear();

        for li in &mg_stored.mg {
            // check if this new lexical item is a head or not
            merge_state = None;
            final_state = None;
            move_hoover = None;
            bundle = &li.bundle;

            // if the first feature is left or right merge, the LI is a head
            if let Some(first_feature) = bundle.first() {
                is_head = matches!(first_feature.rel, LIRelation::LMerge) || matches!(first_feature.rel, LIRelation::RMerge);
            }
            else {
                eprintln!("LI Contains No Features: {}", li.morph);
                continue;
            }
            

            for f in bundle.iter() {
                match f.rel {
                    LIRelation::LMerge | 
                    LIRelation::RMerge | 
                    LIRelation::LMergeHead | 
                    LIRelation::RMergeHead => merge_state = Some(f),

                    LIRelation::PlusMove | 
                    LIRelation::MinusMove => move_hoover = Some(f),

                    LIRelation::State => final_state = Some(f),
                }

                // STEP: ADD ANY STATES (NODES) FROM FEATURE
                // any states found through selectional or categorial features
                // are added as states to our MG
                if matches!(f.rel, LIRelation::LMerge | LIRelation::State | LIRelation::RMerge) 
                && !mg_stored.states.contains(&f.id) {
                    mg_stored.states.insert(f.id.to_string());
                    mg_graph.create_state(&f.id.as_str()).await?;
                }

            }

            println!("Connecting States");
            // there should at least be a final state, either one it becomes after feature checking
            // or one that it currently is with leftover features
            if let Some(state_b) = final_state.take() {
                println!("Setting Property For {}", li.morph);

                // create a relationship between a potential state A and B (handles LIs with selectional features)
                if let Some(state_a) = merge_state.take() {
                    mg_graph.connect_states(&state_a.id, &state_b.id, &li.morph).await?;
                }

                // attach any movement features to the newly created relationship
                if let Some(movement) = move_hoover.take() {
                    if is_head {
                        // heads are represented as a relationship and as such the property
                        // of a relationship is set
                        mg_graph.set_merge_property(&li.morph, "move", &movement.raw).await?;
                    }
                    else {
                        // non-heads are represented as a state and as such the property
                        // of a state / node is set
                        mg_graph.set_state_property("name", &state_b.raw, "move", &movement.raw).await?;
                    }
                }
            }
        }

        // combine nodes which do not need to be separate
        mg_graph.remove_redundancy().await?;

        Ok(mg_graph.clone())
    }
}