use super::graph::GrammarGraph;
use super::mg::MG;
use crate::parse::mg::LIRelation;
use crate::parse::mg::Feature;
use crate::parse::mg::LexicalItem;
use crate::parse::mg::State;
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
            if let Some(morph) = morph_feature_split.first() {
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

                // determine whether we require any intermediate states
                let num_features: usize = individual_feature_split.clone().count();
                let num_merges_required: usize = individual_feature_split.clone()
                    .filter(|c| c.contains("="))
                    .count();
                let requires_intermediate = num_merges_required > 1;

                // STEP 3: iterate over each feature in the LI and add
                // to the feature bundle
                let mut is_last: bool = false; 
                for (i, feature) in individual_feature_split.enumerate() {
                    is_last = i == num_features-1;
                    
                    let (relation, id) = 
                    if feature.starts_with("=>") {
                        // need to create new relation for head merge

                        // determine whether the merge is an intermediate
                        // state or not
                        let relation: LIRelation = if is_last {
                                LIRelation::LMerge
                        } else {
                                LIRelation::LMergeInter
                        };
                        (relation, feature[2..].to_string())
                    }
                    else if feature.starts_with("=") {
                        let relation: LIRelation = if is_last {
                                LIRelation::LMerge
                        } else {
                                LIRelation::LMergeInter
                        };
                        (relation, feature[1..].to_string())
                    } else if feature.ends_with("=") {
                        let relation: LIRelation = if is_last {
                                LIRelation::RMerge
                        } else {
                                LIRelation::RMergeInter
                        };
                        (relation, feature[..feature.len() - 1].to_string())
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
        let mut merge_state: Option<State>;
        let mut final_state: Option<State>; 
        let mut move_hoover: Option<&Feature>;
        let mut intermediate_merge_states: Vec<State> = Vec::new();
        let mut bundle: &Vec<Feature>;
        let mut is_head: bool;
        let mut merge_ids: Vec<String> = Vec::new();

        let mut newest_intermediate_state: Feature;

        mg_stored.states.clear();

        for li in &mg_stored.mg {
            // check if this new lexical item is a head or not
            move_hoover = None;
            final_state = None;
            merge_state = None;

            bundle = &li.bundle;

            // add the LI as the first MERGE id
            merge_ids.push(li.clone().morph);

            // if the first feature is left or right merge, the LI is a head
            if let Some(first_feature) = bundle.first() {
                is_head = matches!(first_feature.rel, LIRelation::LMerge) || 
                    matches!(first_feature.rel, LIRelation::RMerge) || 
                    matches!(first_feature.rel, LIRelation::LMergeInter) ||
                    matches!(first_feature.rel, LIRelation::RMergeInter);
            }
            else {
                eprintln!("LI Contains No Features: {}", li.morph);
                continue;
            }
            
            for (i,f) in bundle.iter().enumerate() {
                
                match f.rel {
                    LIRelation::LMerge | 
                    LIRelation::RMerge | 
                    LIRelation::LMergeHead | 
                    LIRelation::RMergeHead => { 
                        merge_state = Some(State {
                            id: f.id.clone(),
                            is_intermediate: false,
                        })
                    }

                    LIRelation::LMergeInter |
                    LIRelation::RMergeInter => { 
                        intermediate_merge_states.push(State {
                            id: f.id.clone(),
                            is_intermediate: true,
                        });
                        // intermediate states can become
                        // merges

                        // however, the first intermediate state does not
                        // as the first non-head acts as a state which is
                        // already in the derivation
                        // must be more idiomatic way to do this.
                        if i > 0 {
                            merge_ids.push(f.id.clone());
                        }
                    }

                    LIRelation::PlusMove | 
                    LIRelation::MinusMove => move_hoover = Some(f),

                    LIRelation::State => {
                        final_state = Some(State {
                            id: f.id.clone(),
                            is_intermediate: false,
                        });
                    }
                }

                // STEP: ADD ANY STATES (NODES) FROM FEATURE
                // any states found through selectional or categorial features
                // are added as states to our MG
                if matches!(f.rel, LIRelation::LMerge | 
                    LIRelation::State | 
                    LIRelation::RMerge | LIRelation::LMergeInter | LIRelation::RMergeInter) 
                && !mg_stored.states.contains(f.id.as_str()) {
                    mg_stored.states.insert(f.id.to_string());
                    mg_graph.create_state(f.id.as_str()).await?;
                }

            }

            // the first states are the intermediate states, if we have any.
            if !intermediate_merge_states.is_empty() {
                let mut previous: String = String::from("");
                let mut non_head_state: String;
                let mut new_state: String;
                let mut next_merge_li: &str;
                let num_intermediate_states: usize = intermediate_merge_states.iter().count();
                for (i, s) in intermediate_merge_states.iter().enumerate() {

                    
                    if i == num_intermediate_states-1 {
                        break;
                    }
                    // d => <LI.d> => v
                    //
                    // first create an intermediate state <LI.d>
                    // where d is the ID of the first feature.
                    non_head_state = s.id.clone();
                    next_merge_li = merge_ids.get(i).unwrap().as_str();
                    if i == 0 {
                        // if s is the first feature
                        // <HeadLI.LI>
                        new_state = format!("<LI.{}>", non_head_state).clone();
                        mg_graph.create_state(new_state.clone().as_str()).await?;
                        mg_stored.states.insert(new_state.clone());

                        mg_graph.connect_states(&non_head_state, 
                            &new_state, 
                            next_merge_li).await?;
                    }
                    else {
                        // next_merge_li = s.clone();

                        // <<HeadLI.LI>.LI>
                        new_state = format!("<{}.{}>", previous, non_head_state).clone();
                        mg_graph.create_state(new_state.as_str()).await?;

                        // TODO
                        // Q: should inter states be stored as states internally?
                        mg_stored.states.insert(new_state.clone());

                        // the non-head state can be the merge here
                        // as it's being brought into the derivation.
                        // for non-intermediate nodes it's the head
                        // which is being brought into the derivation

                        mg_graph.connect_states(previous.as_str(), 
                            &new_state, 
                            next_merge_li).await?;
                    }

                    // the new state alpha which keeps track of
                    // which state will merge with the final state
                    println!("Setting merge state to {}", s.id.as_str());

                    
                    // TODO: A new struct needs to be created to properly
                    // represent states as states and features are not
                    // the same.
                    merge_state = Some(State {
                        id: new_state.clone(),
                        is_intermediate: true
                    });

                    // update the previous state in case
                    // we need another intermediate node.
                    previous = new_state;
                }

                /*
                merge_state = Feature {
                    id: previous,
                    raw: String::from(""),
                    rel: LIRelation::PlusMove,
                };
                */

                // TODO
                // Need to make sure that when there is intermediate states,
                // the state which is actually merged with the final full state
                // is the right one. Namely, rhe last of the intermediate states.
                // Then, it should be working!
                // It would be worth redoing the logic down below anyway honestly.
            }

            println!("Connecting States");
            // there should at least be a final state, either one it becomes after feature checking
            // or one that it currently is with leftover features
            if let Some(state_b) = final_state.take() {
                println!("Setting Property For {}", li.morph);

                // create a relationship between a potential state A and B (handles LIs with selectional features)
                if let Some(state_a) = merge_state.take() {
                    println!("Connecting {} and {}", state_a.id, state_b.id);
                    let final_merge = merge_ids.last().unwrap();
                    mg_graph.connect_states(&state_a.id, &state_b.id, final_merge).await?;
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
                        mg_graph.set_state_property("name", &state_b.id, "move", &movement.raw).await?;
                    }
                }
            }
        }

        // combine nodes which do not need to be separate
        mg_graph.remove_redundancy().await?;

        Ok(mg_graph.clone())
    }
}
