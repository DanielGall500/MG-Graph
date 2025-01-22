use crate::parse::mg::LexicalItem;
use std::collections::{HashMap, HashSet};

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

    pub fn get_affix_map(&self, mg: &Vec<LexicalItem>) -> HashMap<String, HashSet<usize>> {
        // all morphemes in our MG
        let morphs: Vec<String> = mg.iter().map(|x| x.morph.to_string()).collect();
        let mut pairs: Vec<(String, String)> = Vec::new();

        let mut affix_map: HashMap<String, HashSet<usize>> = HashMap::new();

        let mut affixes: Vec<Affix> = Vec::new();

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
                    affix_map.entry(pre)
                    .or_insert_with(HashSet::new)
                    .extend([i,j].iter());
                }
                if !suff.is_empty() {
                    affix_map.entry(suff)
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
}