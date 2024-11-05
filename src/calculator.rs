use crate::grammar::Grammar;

trait Calculate {
    fn get_grammar_size(&self, mg: &Grammar) -> f64;
}

struct GrammarSizeCalculator;

impl Calculate for GrammarSizeCalculator {
    fn get_grammar_size(&self, mg: &Grammar) -> f64 {
        let mut size: f64 = 0.0;
        let mut n_symbols: usize;
        let mut encoding_cost_per_symbol: f64;

        for (phon, feature_bundle) in mg.set_phon.iter().zip(mg.set_feature_bundles.iter()) {
            let n_phonemes = phon.len(); // number of characters in the phonological representation
            let n_features = feature_bundle.len();
            n_symbols = (n_phonemes + 2 * n_features + 1);
            encoding_cost_per_symbol = (mg.alphabet_size + mg.n_feature_types + mg.get_base_size()) as f64;
            size += n_symbols as f64 * encoding_cost_per_symbol.log2();
        }
        size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let mg_example: &str = "Mary :: da -k; John :: da -k; Mary's :: poss; John's :: poss; the-cause :: d -k;";
        let grammar = match Grammar::new(&mg_example, 26, 7, ';') {
            Ok(g) => g, // If successful, bind the grammar to `g`
            Err(e) => panic!("Failed to create Grammar: {}", e), 
        };
        let calculator: GrammarSizeCalculator = GrammarSizeCalculator;

        let size: f64 = calculator.get_grammar_size(&grammar);
        println!("{}", size);
        assert_eq!(size, 4.0);
    }
}