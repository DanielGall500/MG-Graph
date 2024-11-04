struct Grammar {
    mg: String,
    statements: Vec<String>,
    set_phon: Vec<String>,
    set_feature_bundles: Vec<Vec<String>>
}

impl Grammar {
    fn new(mg: &str, delim: char) -> Self {
        let statements: Vec<String> = Self::mg_to_statements(mg, delim);
        let (set_phon, set_feature_bundles) = Self::statements_to_phon(statements);
        Self {
            mg: mg.to_string(),
            statements: statements,
            set_phon: set_phon,
            set_feature_bundles: set_feature_bundles
        }
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
                set_phon.push(phon.to_string()); 

                // Add δ, the right side of the statement indicating all present features
                let feature_bundle: Vector<String> = syn.split(' ').map(|s| s.to_string());
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

