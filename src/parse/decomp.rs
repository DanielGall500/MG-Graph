use crate::parse::mg::{
    LexicalItem, 
    Feature, 
    LIRelation
};
use std::{
    collections::{HashMap, HashSet},
    cmp::min,
    iter::zip,
    error::Error,
};

pub struct Decomposer {
    pub mg: Vec<LexicalItem>
}

pub enum AffixType {
    PREFIX,
    SUFFIX
}
pub struct Affix {
    pub morph: String,
}

impl Affix {
    pub fn new(morph: &str) -> Self {
        Self {
            morph: morph.to_string(),
        }
    }

    fn get_affix_type(&self) -> Result<AffixType, Box<dyn Error>> {
        if self.morph.starts_with("-") {
            Ok(AffixType::PREFIX)
        }
        else if self.morph.ends_with("-") {
            Ok(AffixType::SUFFIX)
        }
        else {
            Err("Invalid affix: must start or end with '-'".into())
        }
    }
}

impl Decomposer {
    pub fn new() -> Self {
        Self { mg: Vec::new() }
    }

    pub fn decompose(&self, mg: Vec<LexicalItem>, lis_to_decompose: Vec<usize>, affix: Affix, syntax_split_boundary: usize) -> Result<Vec<LexicalItem>, Box<dyn Error>> {
        let mut decomposed_mg: Vec<LexicalItem> = Vec::new();
        let mut decomposed_li: LexicalItem;
        let mut li_morph_decomp: String;
        // dummy affix, improve this
        let mut affix_li = LexicalItem {
            morph: String::from(""),
            bundle: Vec::new(),
        };
        let affix_size: usize = &affix.morph.len()-1; // subtract 1 due to hyphen
        
        /* Ensure affix type is valid, otherwise return unchanged MG. */
        let affix_type = match affix.get_affix_type() {
            Ok(AffixType::PREFIX) => AffixType::PREFIX,
            Ok(AffixType::SUFFIX) =>  AffixType::SUFFIX,
            Err(e) => {
                return Err(e.into())
            }
        };

        // handle decomp
        let mut decomposed_lis: Vec<LexicalItem> = Vec::new();
        for (i, li_index) in lis_to_decompose.iter().enumerate() {
            if let Some(li) = mg.get(li_index.clone()) {
                println!("Operating on LI: {}", li.morph);
                let mut bundle = li.bundle.clone();

                match affix_type {
                    AffixType::PREFIX =>  li_morph_decomp = li.morph[affix_size-1..].to_string(),
                    AffixType::SUFFIX =>  li_morph_decomp = li.morph[0..li.morph.len()-affix_size].to_string(),
                }

                // should move elements to a new bundle but need to check
                let mut affix_bundle = bundle.split_off(syntax_split_boundary);

                // HANDLE AFFIX CASE
                if i == 0 {
                    affix_bundle.insert(0, Feature {
                        raw: format!("=>STATE:{}", affix.morph),
                        id: format!("=>STATE-{}", affix.morph),
                        rel: LIRelation::State
                    });

                    affix_li = LexicalItem {
                        morph: affix.morph.clone(),
                        bundle: affix_bundle
                    };
                }

                // add the new state to the feature bundle of the split root
                bundle.push(Feature {
                    raw: format!("STATE:{}", affix.morph),
                    id: format!("STATE-{}", affix.morph),
                    rel: LIRelation::State
                });

                decomposed_li = LexicalItem {
                    morph: li_morph_decomp,
                    bundle: bundle
                };
                decomposed_lis.push(decomposed_li.clone());

                for i in decomposed_lis.iter() {
                    println!("{}", i.morph);
                }
                println!("---");
            }
        }

        // copy the original mg into a new vector
        decomposed_mg = mg.into_iter().clone().collect();

        // begin by pushing the affix LI
        decomposed_mg.push(affix_li);

        // replace the original root LIs
        for (li_index, decomp_li) in zip(lis_to_decompose, decomposed_lis) {
            println!("Working on {}", decomp_li.morph);
            if let Some(element) = decomposed_mg.get_mut(li_index) {
                // dereferences to modify the value at the actual index
                *element = decomp_li;
                println!("Setting element");
            }
        }

        Ok(decomposed_mg)
    }

    pub fn get_decompose_suggestions(&self, mg: &Vec<LexicalItem>) -> HashMap<String, Vec<usize>> {
        let candidate_set = self.find_decomposition_candidates(mg);
        let mut candidate_set_threshold: HashMap<String, Vec<usize>> = HashMap::new();

        for (affix, mut lis) in candidate_set.into_iter() {
            let total_sim: f64 = lis.iter().map(|(x,y)| y).sum();
            let count = lis.len();
            let mean_sim = total_sim / count as f64;

            let variance = lis.iter().map(|(x,y)| {
                let diff = mean_sim - (*y as f64);

                diff * diff
            }).sum::<f64>() / count as f64;
            let std_dev: f64 = variance.sqrt();

            // mean+α⋅std
            let alpha = 1.0;
            let threshold = mean_sim + (alpha * std_dev);

            let li_final_candidates: Vec<usize> = lis.into_iter()
            .filter(|(x,y)| y >= &threshold)
            .map(|(x,y)| x)
            .collect();

            candidate_set_threshold.insert(affix, li_final_candidates);
        }
        candidate_set_threshold

    }

    pub fn find_decomposition_candidates(&self, mg: &Vec<LexicalItem>) -> HashMap<String, Vec<(usize, f64)>> {
        let affix_map: HashMap<String, HashSet<usize>> = self.get_affix_map(&mg);

        let empty_li = LexicalItem {
            morph: String::from(""),
            bundle: Vec::new()
        };

        let mut candidate_map: HashMap<String, Vec<(usize, f64)>> = HashMap::new();
        let mut avg_sim: f64;
        let mut total_sim: f64;
        let mut count: usize;
        let mut li_similarity_vec: Vec<f64>;

        for (affix, li_indices) in affix_map.iter() {

            for i in li_indices.iter() {
                if let Some(affix_li) = mg.get(i.clone()) {

                    /* Bug: Includes similarity to itself. */
                    li_similarity_vec = li_indices.iter()
                    .map(|x| self.get_syntactic_similarity(mg.get(x.clone()).unwrap_or(&empty_li), affix_li)).collect();

                    println!("Similarities of {}:", affix_li.morph);
                    for s in &li_similarity_vec {
                        println!("{}", s);
                    }
                    println!("----");

                    total_sim = li_similarity_vec.iter().sum(); 
                    count = li_similarity_vec.len();
                    avg_sim = total_sim / count as f64;

                    candidate_map.entry(affix.clone())
                    .or_insert_with(Vec::new)
                    .push((i.clone(), avg_sim));
                }
            }

        }

        // SORT THE ITEMS IN DESCENDING ORDER
        // candidate_map.iter().map(|(s,v)| v.sort_by_key(|(index, sim)| sim)).collect();
        let mut sorted_candidate_map: HashMap<String, Vec<(usize, f64)>> = HashMap::new();
        for (affix, mut lis) in candidate_map.into_iter() {
              lis.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap()); 
              sorted_candidate_map.insert(affix, lis);
        }

        sorted_candidate_map
    }

    pub fn get_syntactic_similarity(&self,  l1: &LexicalItem, l2: &LexicalItem) -> f64 {
        let fb1: Vec<String> = l1.bundle.iter().map(|x| x.raw.clone()).collect();
        let fb2: Vec<String> = l2.bundle.iter().map(|x| x.raw.clone()).collect();
        let mut similarity: f64 = 0.0;
        let n: usize;

        let kronecker_delta = |x: bool| -> f64 {
            x as u16 as f64
        };

        let is_active_feature_same = &fb1[0] == &fb2[0];
        if is_active_feature_same {
            n = min(fb1.len(), fb2.len());
            let alpha: f64 = 2.0;
            let mut f1: &String;
            let mut f2: &String;
            let mut wi: f64;

            // start from the first non-active feature (the second overall feature)
            for i in 1..n {
                f1 = &fb1[i];
                f2 = &fb2[i];
                wi = (-alpha * ((i-1) as f64)).exp();
                println!("Updating Similarity with weight {} and delta {}", wi, kronecker_delta(f1 == f2));
                // does the n normalise to some extent?
                similarity += (kronecker_delta(f1 == f2)) / n as f64;
            }
        }
        similarity
    }

    pub fn get_affix_map(&self, mg: &Vec<LexicalItem>) -> HashMap<String, HashSet<usize>> {
        // all morphemes in our MG
        let morphs: Vec<String> = mg.iter().map(|x| x.morph.to_string()).collect();
        let mut affix_map: HashMap<String, HashSet<usize>> = HashMap::new();

        /* TODO: OPTMISATION PROCESS */
        // all pairs in our MG
        let mut j: usize; 
        for (i, m1) in morphs.iter().enumerate() {
            j = i+1;
            for m2 in &morphs[j..] {
                let (pre, suff) = self.get_common_affix(&m1, &m2);

                // creates a dictionary which stores each affix
                // and the lexical item index to which it is associated.
                if !pre.is_empty() {
                    affix_map.entry(format!("{pre}-"))
                    .or_insert_with(HashSet::new)
                    .extend([i,j].iter());
                }
                if !suff.is_empty() {
                    affix_map.entry(format!("-{suff}"))
                    .or_insert_with(HashSet::new)
                    .extend([i,j].iter());
                }
                j += 1;
            }
        }
        affix_map
    }


    pub fn get_common_affix(&self, morph1: &str, morph2: &str) -> (String, String) {
        let common_prefix: String = morph1
            .chars()
            .zip(morph2.chars())
            .take_while(|(m1_char, m2_char)| m1_char == m2_char)
            .map(|(m1_char, _)| m1_char)
            .collect();

        let mut common_suffix: String = morph1
            .chars()
            .rev()
            .zip(morph2.chars().rev())
            .take_while(|(m1_char, m2_char)| m1_char == m2_char)
            .map(|(m1_char, _)| m1_char)
            .collect();

        // the suffix is returned in reverse, so this is fixed by reversing it again.
        common_suffix = common_suffix.chars().rev().collect();

        // let has_prefix = !common_prefix.is_empty();
        // let has_suffix = !common_suffix.is_empty();
        (common_prefix, common_suffix)
    }

}

pub fn test_decompose_affix_finder(mg: &Vec<LexicalItem>) {
    let decomp: Decomposer = Decomposer::new();
    let affix_map = decomp.get_affix_map(&mg);
    println!("Affix Map:");
    for (key, values) in &affix_map {
        println!("{}: {:?}", key, values);
    }

    println!("Test Similarity Calculator");
    if let Some(li1) = mg.get(1) {
        if let Some(li2) = mg.get(2) {
            let similarity = decomp.get_syntactic_similarity(&li1, &li2);
            println!("Similarity between {} and {} is {}", li1.morph, li2.morph, similarity);
        }
    }

    let candidate_map = decomp.find_decomposition_candidates(&mg);
    println!("Candidate Map:");
    for (key, values) in &candidate_map {
        println!("Affix: {}", key);
        for (index, sim) in values.iter() {
            if let Some(li) = mg.get(index.clone()) {
                // println!("Avg Sim of {} is {} for affix {}", li.morph, sim, key);
                println!("{},{}", li.morph, sim);
            }
        }
        println!("---");
    }

    let decomp_suggestions = decomp.get_decompose_suggestions(mg);
    let decompose_choice = decomp_suggestions.get("-s").unwrap();
    println!("Suggestions for decomp:");
    for i in decompose_choice {
        println!("* {}", mg.get(i.clone()).unwrap().morph);
    }
    println!("----");
    let affix = Affix {
        morph: String::from("-s"),
    };
    let k = 1;
    let decomposed_mg = decomp.decompose(mg.clone(), decompose_choice.clone(), affix, k);
    
    use crate::parse::mg::MGParser;
    let mut parser = MGParser::new();
    parser.update_grammar(decomposed_mg.unwrap());
    parser.to_json("decomposed_suffix").unwrap();

}