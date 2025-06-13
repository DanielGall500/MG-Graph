use super::graph::GrammarGraph;
use super::mg::MG;
use crate::parse::mg::LIRelation;
use crate::parse::mg::Feature;
use crate::parse::mg::LexicalItem;
use crate::parse::mg::State;
use std::collections::HashMap;
use std::error::Error;
use crate::cypher::cquery::{Node, Relationship};

pub fn get_node(label: String, state_type: String) -> Node {
    let mut basic_node_props : HashMap<String, String> = HashMap::new();
    basic_node_props.insert(String::from("move"), String::from(""));
    Node {
        state_type: state_type,
        label: label,
        props: Some(basic_node_props)
    }
}

pub fn get_default_node(label: String) -> Node {
    let state_type = String::from("State");
    get_node(label, state_type)
}

pub fn get_intermediate_node(label: String) -> Node {
    let state_type = String::from("Interm");
    get_node(label, state_type)
}

pub fn get_default_relationship(node_a: Node, node_b: Node, li: String) -> Relationship {
    let mut basic_rel_props : HashMap<String, String> = HashMap::new();
    basic_rel_props.insert(String::from("move"), String::from(""));
    Relationship {
        node_a,
        node_b,
        li,
        props: basic_rel_props
    }
}

pub struct Parser {}

impl Parser {
    pub fn convert_text_to_stored(minimalist_grammar: &str, mg_stored: &mut MG) -> Result<(), Box<dyn Error>> {
        mg_stored.mg.clear();
        mg_stored.states.clear();
        let mut li: LexicalItem;

        let mg_statements = minimalist_grammar
            .split(";")
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
                let num_features: i8 = individual_feature_split.clone().count() as i8;
                let num_merges_required: i8 = individual_feature_split.clone()
                    .filter(|c| c.contains("="))
                    .count() as i8;

                let num_movement_features: i8 = individual_feature_split.clone()
                    .filter(|c| c.contains("+") || c.contains("-"))
                    .count() as i8;

                let requires_intermediate = num_merges_required > 1;


                // STEP 3: iterate over each feature in the LI and add
                // to the feature bundle
                let mut is_last_selec: bool; 
                for (i, feature) in individual_feature_split.enumerate() {
                    is_last_selec = i as i8 - num_movement_features == 
                        num_features-num_movement_features-1;
                    
                    let (relation, id) = 
                    if let Some(stripped) = feature.strip_prefix("=>") {
                        // need to create new relation for head merge

                        // determine whether the merge is an intermediate
                        // state or not
                        let relation: LIRelation = if is_last_selec || !requires_intermediate {
                                LIRelation::LMerge
                        } 
                        else {
                                LIRelation::LMergeInter
                        };
                        (relation, stripped.to_string())
                    }
                    else if let Some(stripped) = feature.strip_prefix("=") {
                        let relation: LIRelation = if is_last_selec || !requires_intermediate {
                                LIRelation::LMerge
                        } else {
                                LIRelation::LMergeInter
                        };
                        (relation, stripped.to_string())
                    } else if feature.ends_with("=") {
                        let relation: LIRelation = if is_last_selec || !requires_intermediate {
                                LIRelation::RMerge
                        } else {
                                LIRelation::RMergeInter
                        };
                        (relation, feature[..feature.len() - 1].to_string())
                    } else if let Some(stripped) = feature.strip_prefix("-") {
                        (LIRelation::MinusMove, stripped.to_string())
                    } else if let Some(stripped) = feature.strip_prefix("+") {
                        (LIRelation::PlusMove, stripped.to_string())
                    } else {
                        (LIRelation::State, feature.clone())
                    };

                    // STEP 4: ADD FEATURE INFO TO LI
                    li.bundle.push(Feature {
                        raw: feature.clone(),
                        id: id.clone(),
                        rel: relation,
                    });

                    println!("Valid -{}-", feature);
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
        let mut merge_state_indx: usize;
        let mut final_state: Option<State>; 
        let mut intermediate_merge_states: Vec<State> = Vec::new();
        let mut bundle: &Vec<Feature>;
        let mut is_head: bool;
        let mut merge_ids: Vec<String> = Vec::new();

        mg_stored.states.clear();

        for li in &mg_stored.mg {
            let mut all_states: Vec<State> = Vec::new();
            let mut total_merges: usize = 0;

            // check if this new lexical item is a head or not
            final_state = None;
            // defaults to first
            merge_state_indx = 0;

            bundle = &li.bundle;

            // add the LI as the first MERGE id
            merge_ids.push(li.clone().morph);

            println!("Working on LI {}", li.morph);

            // if the first feature is left or right merge, the LI is a head
            // we skip over adding non-heads until they appear in an LI
            // TODO: Don't skip it all together
            if let Some(first_feature) = bundle.first() {
                is_head = matches!(first_feature.rel, LIRelation::LMerge) || 
                    matches!(first_feature.rel, LIRelation::RMerge) || 
                    matches!(first_feature.rel, LIRelation::LMergeInter) ||
                    matches!(first_feature.rel, LIRelation::RMergeInter);
                println!("LI is head? {}", is_head);
            }
            else {
                eprintln!("LI Contains No Features: {}", li.morph);
                continue;
            }
            
            // iterate over the features of this LI by (index, feature)
            for (i,f) in bundle.iter().enumerate() {
                
                match f.rel {
                    LIRelation::LMerge | 
                    LIRelation::RMerge | 
                    LIRelation::LMergeHead | 
                    LIRelation::RMergeHead => { 
                        // laugh :: *=v* +k t;
                        total_merges += 1;
                        let new_state = State {
                            id: f.id.clone(),
                            is_intermediate: false,
                            moves: Vec::new()
                        };
                        all_states.push(new_state.clone());
                    }

                    LIRelation::LMergeInter |
                    LIRelation::RMergeInter => { 
                        // laugh at :: *=v* =v +k t;
                        total_merges += 1;

                        // we must update which state will be
                        // connected with the final state.
                        merge_state_indx = total_merges-1;

                        let new_state = State {
                            id: f.id.clone(),
                            is_intermediate: true,
                            moves: Vec::new()
                        };
                        intermediate_merge_states.push(new_state.clone());
                        all_states.push(new_state.clone());
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
                    LIRelation::MinusMove => {
                        // append a movement feature to the most
                        // recent state
                        println!("Appending move feature {}", f.id);
                        if let Some(recent_op) = all_states.last_mut() {
                            recent_op.moves.push(f.raw.clone());
                        }
                    }

                    LIRelation::State => {
                        let new_state = State {
                            id: f.id.clone(),
                            is_intermediate: false,
                            moves: Vec::new()
                        };
                        final_state = Some(new_state.clone());
                        all_states.push(new_state.clone());
                    }
                }

                // STEP: ADD ANY STATES (NODES) FROM FEATURE
                // any states found through selectional or categorial features
                // are added as states to our MG
                //
                // TODO: 
                // Extra states are being created here.
                if matches!(f.rel, LIRelation::LMerge | 
                    LIRelation::State | 
                    LIRelation::RMerge) 
                && !mg_stored.states.contains(f.id.as_str()) {
                    mg_stored.states.insert(f.id.to_string());

                    mg_graph.create_state(
                        get_default_node(f.id.clone())
                    ).await?;
                }

            }

            println!("Number of Total States: {}", all_states.len());

            let mut previous: String = String::from("");
            let mut non_head_state: String;
            let mut new_state: String;
            let num_states_in_li: usize = all_states.len();

            // iterate over the states which are connected for this
            // specific LI and connect them.
            for (i, s) in all_states.iter().enumerate() {
                let is_intermediate = s.is_intermediate;

                println!("Working on state {}", s.id);

                if num_states_in_li == 1 {
                    for m in s.moves.iter() {
                        println!("Setting the move property of that state");
                        mg_graph.set_state_property( s.id.as_str(), "move", m).await?;
                    }
                }
                // laughs :: =d +k v; Mary :: d -k
                // IS FIRST AND NOT INTERMEDIATE
                else if i == 0 && !is_intermediate {
                    if let Some(output_state) = final_state.take() {

                        let first_state: Node = get_default_node(s.id.clone());
                        let second_state: Node = get_default_node(output_state.id.clone());
                        let connection: Relationship = get_default_relationship(
                            first_state, 
                            second_state, 
                            li.morph.clone()
                        );
                        // connect the current state and the output state
                        mg_graph.connect_states(connection).await?;

                        let all_moves: String = s.moves.join(",");
                        mg_graph.set_merge_property(&li.morph, "move", all_moves.as_str()).await?;
                    }
                }
                // first operation of multiple
                // IS FIRST AND INTERMEDIATE
                else if i == 0 {
                    // if s is the first feature
                    // <HeadLI.LI>
                    non_head_state = s.id.clone();
                    new_state = format!("<LI.{}>", s.id);

                    // for the active feature, we must make sure there is 
                    // a node for both the non-head and head.
                    // The below creates a node **if none exists**.
                    // Note: This may lead to some issues and should
                    // be more properly defined.
                    
                    // mg_graph.create_state(non_head_state.as_str()).await?; //redundant?
                    mg_graph.create_state(
                        get_intermediate_node(new_state.clone())
                    ).await?;

                    // make this automatic
                    mg_stored.states.insert(new_state.clone().to_string());

                    println!("Connecting two states...");
                    println!("{}{}", non_head_state, new_state);
                        let first_state: Node = get_default_node(non_head_state);
                        let second_state: Node = get_intermediate_node(new_state.clone());
                        let connection: Relationship = get_default_relationship(
                            first_state, 
                            second_state, 
                            li.morph.clone()
                        );
                    mg_graph.connect_states(connection).await?;

                    let all_moves: String = s.moves.join(",");
                    mg_graph.set_merge_property(&li.morph, "move", all_moves.as_str()).await?;
                    
                    previous = new_state.to_string().clone();
                }
                // TIME TO LEAVE INTERMEDIATE STATES
                else if i == merge_state_indx && is_intermediate {
                    // <<HeadLI.LI>.LI>
                    // new_state = format!("<{}.{}>", previous, s.id);

                    // TODO
                    // Q: should inter states be stored as states internally?
                    // mg_stored.states.insert(new_state.clone().to_string());

                    // the non-head state can be the merge here
                    // as it's being brought into the derivation.
                    // for non-intermediate nodes it's the head
                    // which is being brought into the derivation

                    if let Some(output_state) = final_state.take() {
                        let final_intermediate_node: Node = get_intermediate_node(previous.clone());
                        let output_node: Node = get_default_node(output_state.id);
                        let connection: Relationship = get_default_relationship(
                            final_intermediate_node, 
                            output_node, 
                            s.id.clone()
                        );
                        mg_graph.connect_states(connection).await?;

                        let all_moves: String = s.moves.join(",");
                        mg_graph.set_merge_property(s.id.as_str(), "move", all_moves.as_str()).await?;
                    }
                }
                // NOT FIRST AND INTERMEDIATE
                else if is_intermediate {

                    // <<HeadLI.LI>.LI>
                    println!("Is Intermediate!");
                    new_state = format!("<{}.{}>", previous, s.id);
                    println!("Creating state {}", new_state);

                    mg_graph.create_state(
                        get_intermediate_node(new_state.clone())
                    ).await?;

                    // TODO
                    // Q: should inter states be stored as states internally?
                    mg_stored.states.insert(new_state.clone().to_string());

                    // the non-head state can be the merge here
                    // as it's being brought into the derivation.
                    // for non-intermediate nodes it's the head
                    // which is being brought into the derivation

                    let intermediate_node_a: Node = get_intermediate_node(previous.clone());
                    let intermediate_node_b: Node = get_intermediate_node(new_state.clone());
                    let connection: Relationship = get_default_relationship(
                        intermediate_node_a, 
                        intermediate_node_b, 
                        s.id.clone()
                    );
                    mg_graph.connect_states(connection).await?;

                    for m in s.moves.iter() {
                        // heads are represented as a relationship and as such the property
                        // of a relationship is set
                        mg_graph.set_state_property(new_state.as_str(), "move", m).await?;
                    }
                    previous = new_state.to_string().clone();
                }

            }
        }

        // combine nodes which do not need to be separate
        mg_graph.remove_redundancy().await?;

        Ok(mg_graph.clone())
    }
}
