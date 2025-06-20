use crate::parse::grammar::Grammar;

#[derive(serde::Serialize)]
pub struct MDL {
    n_features: usize,
    n_phonemes: usize,
    encoding_cost_per_symbol: f64,
    mdl: f64
}

pub trait Calculate {
    fn get_grammar_size(&self, mg: &Grammar, verbose: bool) -> MDL;
}

pub struct GrammarSizeCalculator;

impl Calculate for GrammarSizeCalculator {
    fn get_grammar_size(&self, mg: &Grammar, verbose: bool) -> MDL {
        let mut n_symbols: f64 = 0.0;
        let mut n_features: usize = 0;
        let mut n_phonemes: usize = 0;

        for (phon, feature_bundle) in mg.set_phon.iter().zip(mg.set_feature_bundles.iter()) {
            let n_phonemes_i = phon.len(); // number of characters in the phonological representation
            let n_features_i = feature_bundle.len();

            n_features += n_features_i;
            n_phonemes += n_phonemes_i;

            let n_symbols_i = n_phonemes_i + 2 * n_features_i + 1;
            n_symbols += n_symbols_i as f64;

            if verbose {
                println!("Word: {}", phon);
                feature_bundle.iter().for_each(|f| print!(" {}",f));
                println!("(|{}| + 2 * |{}| + 1)", n_phonemes_i,n_features_i);
            }
        }

        let encoding_cost_per_symbol: f64 = ((mg.alphabet_size + mg.n_feature_types + mg.get_base_size() + 1) as f64).log2();
        if verbose {
            println!("Base Size: {}", mg.get_base_size());
            println!("Encoding Cost Per Symbol: {}", encoding_cost_per_symbol);
        }

        let mdl = n_symbols * encoding_cost_per_symbol;

        MDL {
            n_features,
            n_phonemes,
            encoding_cost_per_symbol,
            mdl
        }

    }
}