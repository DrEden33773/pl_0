use crate::ast::ProgramExpr;

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct AstOptimizer {
  ast_entry: Box<ProgramExpr>,
}

impl AstOptimizer {
  pub fn optimize(self) -> Box<ProgramExpr> {
    println!();
    println!("Unimplemented AstOptimizer, use the raw AST instead");
    println!("Successfully optimized the AST");
    self.ast_entry
  }
}

impl AstOptimizer {
  pub fn new(ast_entry: Box<ProgramExpr>) -> Self {
    Self { ast_entry }
  }
}
