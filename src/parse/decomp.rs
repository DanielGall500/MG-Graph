use crate::parse::mg::LexicalItem;
use std::collections::{HashMap, HashSet};
use std::cmp::min;
use std::iter::zip;

pub struct Decomposer {
    mg: Vec<LexicalItem>
}

pub enum AffixType {
    PREFIX,
    SUFFIX
}
pub struct Affix {
    morph: String,
    affix_type: AffixType,
    lexical_item: usize
}

impl Decomposer {
    pub fn new() -> Self {
        Self { mg: Vec::new() }
    }

    pub fn decompose() {

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
        candidate_map
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
        for (index, sim) in values.iter() {
            if let Some(li) = mg.get(index.clone()) {
                println!("Avg Sim of {} is {} for affix {}", li.morph, sim, key);
            }
        }
    }
}