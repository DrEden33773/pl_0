#![allow(unused)]

pub mod desc;

use self::desc::{ActivationRecord, Level};
use crate::{ast::ProgramExpr, lexer::Lexer};

struct CodegenContext<'a> {
  /// Marking original code (for better error handling)
  lexer: Lexer<'a>,
  /// Levels of current context
  levels: Vec<Level>,
}

impl<'a> CodegenContext<'a> {
  fn new(ctx: &'a str) -> Self {
    Self {
      lexer: Lexer::new(ctx),
      levels: vec![],
    }
  }
}

pub struct TreeWalkCodeGenerator<'a> {
  /// Marking original code (for better error handling)
  ctx: CodegenContext<'a>,
  /// The only legal entrance ast_node
  ast_entry: Box<ProgramExpr>,
  /// Aka. stack pointer
  sp: usize,
  /// Aka. activation_record
  ar: ActivationRecord,
}

impl<'a> TreeWalkCodeGenerator<'a> {
  pub fn new(ctx: &'a str, ast_entry: Box<ProgramExpr>) -> Self {
    Self {
      ctx: CodegenContext::new(ctx),
      ast_entry,
      sp: 0,
      ar: ActivationRecord::default(),
    }
  }
}
