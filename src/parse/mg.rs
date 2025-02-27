use serde::{Serialize, Deserialize};
use std::io::{BufReader, Write};
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::fmt;


#[derive(Serialize, Deserialize, Clone)]
pub struct State {
    pub id: String,
    pub is_intermediate: bool
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Feature {
    pub raw: String,
    pub id: String,
    pub rel: LIRelation
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LexicalItem {
    pub morph: String,
    pub bundle: Vec<Feature>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum LIRelation {
    LMerge, // =x
    RMerge, // x= 
    LMergeInter, // =x where it there is more than one merge
    RMergeInter, // x= where there is more than one merge
    LMergeHead, // =>x
    RMergeHead, // x<=
    MinusMove, // -x
    PlusMove, // +x
    State, // x
}

/* Create the basic data structure for storing an MG. */
pub struct MG {
    pub mg: Vec<LexicalItem>,
    pub states: HashSet<String>
}

impl MG {
    pub fn new() -> Self {
        Self {
            mg: Vec::new(),

            // states are updated automatically when new MGs are added
            // to the graph. Note they do not update upon running update_grammar
            states: HashSet::new(),
        }
    }

    pub fn get_grammar(&self) -> &Vec<LexicalItem> {
        &self.mg
    }

    pub fn update_grammar(&mut self, updated_mg: Vec<LexicalItem>) {
        self.mg = updated_mg;
    }

    pub fn to_json(&self, title: &str) -> Result<(), Box<dyn Error>> {
        let path: String = format!("./parse/grammar_parsed_{}.json", title);
        let mut file = File::create(path)?;
        match serde_json::to_string_pretty(&self.mg) {
            Ok(json) => file.write_all(json.as_bytes())?,
            Err(e) => eprintln!("Error serializing data to JSON: {}", e),
        }
        Ok(())
    }

    pub fn from_json(&self, title: &str) -> Result<Vec<LexicalItem>, Box<dyn Error>> {
        let path: String = format!("./parse/grammar_parsed_{}.json", title);
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let config: Vec<LexicalItem> = serde_json::from_reader(reader)?;
        Ok(config)
    }
}

impl fmt::Display for MG {
    /*
    TODO:
    - Check that the below outputs the MG correctly.
    - Change Parser to MG simply.
    - Update textbox with current MGs. */
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut mg_as_str: String = String::from("");
        for li in self.mg.iter() {
            let m = li.morph.clone(); // should this be &str?
            let fb = li.bundle.clone();
            let fb_as_str: String = fb.iter().map(|x| x.raw.clone()).collect::<Vec<String>>().join(" ");

            let li_line = format!("{} :: {};\n", m.as_str(), fb_as_str.as_str());
            mg_as_str.push_str(li_line.as_str()); 
        }
        println!("{}", mg_as_str);
        write!(f, "{}", mg_as_str)
    }
}
