pub struct Grammar {
    mg: String,
    statements: Vec<String>,
    pub alphabet_size: usize,
    pub n_feature_types: usize,
    pub set_phon: Vec<String>,
    pub set_feature_bundles: Vec<Vec<String>>
}

pub impl Grammar {
    pub fn new(mg: &str, alphabet_size: usize, n_feature_types: usize, delim: char) -> Result<Self,&'static str> {
        let statements: Vec<String> = Self::mg_to_statements(mg, delim);
        let (set_phon, set_feature_bundles) = Self::statements_to_phon(statements);

        if set_phon.len() == set_feature_bundles.len() {
            Ok(Self {
                mg: mg.to_string(),
                statements: statements,
                alphabet_size: alphabet_size,
                n_feature_types: n_feature_types,
                set_phon: set_phon,
                set_feature_bundles: set_feature_bundles
            })
        }
        else {
            Err("Number of items and feature bundles must match.")
        }
    }

    pub fn get_base_size(&self) -> usize {
        self.statements.len()
    }

    pub fn get_phon_size(&self) -> usize {
        self.phon.len()
    }

    pub fn get_feature_size(&self) -> usize {
        get_flattened_size(self.feature_bundles)
    }

    fn mg_to_statements(mg: &str, delim: char) -> Vec<String> {
        let result: Vec<String> = mg.split(delim)
            .map(|s| s.to_string())
            .collect();

        result
    }

    fn statements_to_sets(statements: Vec<String>) -> Vec<String> {
        let mut set_phon: Vec<String> = Vec::new();
        let mut set_feature_bundle: Vec<Vec<String>> = Vec::new();

        for statement in statements {
            if let Some((phon, syn)) = statement.split_once("::") {
                // Add s, the left side of the statement indicating the LI's phonological realisation
                let phon: String = phon.trim().to_string();
                set_phon.push(phon); 

                // Add δ, the right side of the statement indicating all present features
                let feature_bundle: Vec<String> = syn.split(' ').map(|s| s.trim().to_string());
                set_feature_bundle.push(feature_bundle)
            } else {
                println!("Statement formatted incorrectly. Ignoring.");
                continue
            }
        }

        // s (phonology) :: δ (feature bundle)
        (set_phon, set_syn)
    }
}
