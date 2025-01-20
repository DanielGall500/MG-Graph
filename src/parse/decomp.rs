use std::iter::zip;
use crate::parse::mg::LexicalItem;

pub struct Decomposer {
    mg: Vec<LexicalItem>
}

impl Decomposer {
    pub fn new() -> Self {
        Self { mg: Vec::new() }
    }

    pub fn decompose() {

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

pub fn test_decompose_affix_finder() {
    let item: Decomposer = Decomposer::new();
    let (pre, suff) = item.get_common_affix("bedanken", "bekommen");
    println!("Prefix: {}, Suffix: {}", pre, suff);
    assert_eq!(pre, "be");
}