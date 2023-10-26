use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "pest_parser/grammar.pest"]
pub struct PestParser {}
