extern crate pest;

use pest::{Parser, ParserState, ParseResult,};
use pest_derive;


#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct DivParser;

impl DivParser {

    pub fn get_pairs(input: String) -> () {
        let words = Self::parse(Rule::name, input.as_str())
            .expect("Could not parse string")
            .next().unwrap();
        println!("{:#?}", words);
    }
}
