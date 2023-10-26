use pest::Parser;
use pest_derive::Parser;

use crate::ast::ProgramExpr;

#[derive(Parser)]
#[grammar = "pest_parser/grammar.pest"]
pub struct PestParser {
  pub ast_entry: Option<Box<ProgramExpr>>,
}

impl PestParser {
  pub fn parse_content(str: &str) -> Option<Box<ProgramExpr>> {
    let _pairs = PestParser::parse(Rule::prog, str);
    None
  }
}
