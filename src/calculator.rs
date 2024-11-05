use grammar::Grammar;

trait Calculator {
    fn get_grammar_size(&self) -> usize;
}

pub impl Calculator {
    pub fn get_grammar_size(&self, mg: Grammar) -> usize {
        let mut size: usize = 0;
        let mut n_symbols: usize;
        let mut encoding_cost_per_symbol: usize;

        for (phon, feature_bundle) in set_phon.iter().zip(set_feature_bundles.iter()) {
            n_phonemes = phon.len(); // number of characters in the phonological representation
            n_features = feature_bundle.len();
            n_symbols = (n_phonemes + 2 * n_features + 1);
            encoding_cost_per_symbol = mg.alphabet_size + mg.n_feature_types + mg.get_base_size();
            size += n_symbols * encoding_cost_per_symbol.log2();
        }
        size
    }
}

fn get_flattened_size(matrix: Vec<Vec<T>>) -> usize {
    matrix.iter().map(|bundle| bundle.len()).sum()
}