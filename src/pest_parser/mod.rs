use crate::ast::ProgramExpr;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "pest_parser/grammar.pest"]
pub struct PestParser {
  pub sat_entry: Option<Box<ProgramExpr>>,
}

impl PestParser {
  pub fn parse_content(str: &str) -> Option<Box<ProgramExpr>> {
    let _pairs = PestParser::parse(Rule::prog, str);
    None
  }
}
