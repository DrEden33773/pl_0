#![allow(unused)]

pub mod desc;

use self::desc::ActivationRecord;
use crate::{ast::ProgramExpr, lexer::Lexer};

pub struct TreeWalkCodeGenerator<'a> {
  /// Marking original code (for better error handling)
  lexer: Lexer<'a>,
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
      lexer: Lexer::new(ctx),
      ast_entry,
      sp: 0,
      ar: ActivationRecord::default(),
    }
  }
}
