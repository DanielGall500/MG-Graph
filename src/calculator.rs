trait Calculator {
    fn get_base_size(&self) -> usize;
    fn get_phon_size(&self) -> usize;
    fn get_feature_size(&self) -> usize;
    fn get_grammar_size(&self) -> usize;
}

impl Calculator {
    fn get_base_size(&self) -> usize {
        self.statements.len()
    }

    fn get_phon_size(&self) -> usize {
        self.phon.len()
    }

    fn get_feature_size(&self) -> usize {
        get_flattened_size(self.feature_bundles)
    }

    fn get_grammar_size(&self) -> usize {
        /* TODO */
    }
}

fn get_flattened_size(matrix: Vec<Vec<T>>) -> usize {
    matrix.iter().map(|bundle| bundle.len()).sum()
}