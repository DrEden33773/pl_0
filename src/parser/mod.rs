#![allow(dead_code)]

mod methods;

use crate::{
  ast::ProgramExpr,
  error::compile_error::CompileError,
  lexer::{token_def::Token, Lexer, LexerIterator},
  SEP,
};

impl<'a> Lexer<'a> {
  fn consume_next(&mut self, token: Token) {
    if let Some(t) = self.next() {
      if let Token::Error(err) = t.to_owned() {
        eprintln!("{}", err);
      }
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
      if let Token::Error(err) = t.to_owned() {
        eprintln!("{}", err);
        false
      } else {
        *t == token
      }
    } else {
      self.panic_compile_error(
        CompileError::syntax_error_template(),
        format!("Expected `{:?}`, but got `None`", token),
      );
      // false
    }
  }

  fn match_next_identifier(&mut self) -> (bool, String) {
    if let Some(t) = self.peek() {
      if let Token::Error(err) = t.to_owned() {
        eprintln!("{}", err);
        (false, String::new())
      } else if let Token::Identifier(id) = t {
        (true, id.to_owned())
      } else {
        (false, String::new())
      }
    } else {
      self.panic_compile_error(
        CompileError::syntax_error_template(),
        "Expected <id>, but got `None`".to_string(),
      );
    }
  }

  fn match_next_integer(&mut self) -> (bool, i64) {
    if let Some(t) = self.peek() {
      if let Token::Error(err) = t.to_owned() {
        eprintln!("{}", err);
        (false, 0)
      } else if let Token::Integer(num) = t {
        (true, num.to_owned())
      } else {
        (false, 0)
      }
    } else {
      self.panic_compile_error(
        CompileError::syntax_error_template(),
        "Expected <num>, but got `None`".to_string(),
      );
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

  pub fn parse(&mut self) -> &mut Self {
    let program_expr = self.parse_program();
    self.ast_entry = Some(program_expr.into());
    self
  }

  pub fn show_ast(&self) {
    println!("AST:");
    println!("{}", SEP.as_str());
    match &self.ast_entry {
      Some(ctx) => println!("{:#?}", ctx),
      None => println!("None"),
    }
  }
}
