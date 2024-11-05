use grammarsize::grammar;
use grammarsize::calculator;
use grammarsize::calculator::Calculate;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_undergeneralised_minimalist_grammar() {
        let mg_example: &str = "Mary :: d -k;
                                laughs :: =d +k t;
                                laughed :: =d +k t;
                                jumps :: =d +k t;
                                jumped :: =d +k t;";
        let grammar = match grammar::Grammar::new(&mg_example, 26, 7, ';') {
            Ok(g) => g, // If successful, bind the grammar to `g`
            Err(e) => panic!("Failed to create Grammar: {}", e), 
        };

        let calculator: calculator::GrammarSizeCalculator = calculator::GrammarSizeCalculator;

        let size: f64 = calculator.get_grammar_size(&grammar, false);
        println!("{}", size);
        assert_eq!(size.round(), 318.0);
    }
}