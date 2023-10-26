#![allow(dead_code)]

mod methods;

use crate::{
  ast::ProgramExpr,
  error::compile_error::CompileError,
  lexer::{token_def::Token, Lexer, LexerIterator},
  SEP,
};

pub(super) type ParseResult = ();

impl<'a> Lexer<'a> {
  fn consume_next(&mut self, token: Token) {
    if let Some(t) = self.next() {
      if t != token {
        self.panic_compile_error(
          CompileError::syntax_error_template(),
          format!("Expected `{:?}`, but got `{:?}`", token, t),
        );
      }
    } else {
      self.panic_compile_error(
        CompileError::syntax_error_template(),
        format!("Expected `{:?}`, but got `None`", token),
      );
    }
  }

  fn match_next(&mut self, token: Token) -> bool {
    if let Some(t) = self.peek() {
      *t == token
    } else {
      self.panic_compile_error(
        CompileError::syntax_error_template(),
        format!("Expected `{:?}`, but got `None`", token),
      );
      false
    }
  }
}

#[derive(Debug)]
pub struct Parser<'a> {
  lexer: Lexer<'a>,
  ast_entry: Option<Box<ProgramExpr>>,
}

impl<'a> Parser<'a> {
  pub fn new(ctx: &'a str) -> Self {
    Self {
      lexer: Lexer::new(ctx),
      ast_entry: None,
    }
  }

  pub fn parse(&mut self) -> ParseResult {
    let program_expr = self.parse_program();
    self.ast_entry = Some(program_expr.into());
  }

  pub fn show_ugly_ast(&self) {
    println!("AST:");
    println!("{}", SEP.as_str());
    match &self.ast_entry {
      Some(ctx) => println!("{:#?}", ctx),
      None => println!("None"),
    }
  }
}
