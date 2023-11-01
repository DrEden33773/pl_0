#![allow(dead_code)]

mod methods;

use crate::{
  ast::ProgramExpr,
  error::compile_error::CompileError,
  lexer::{token_def::Token, Lexer, LexerIterator},
  SEP,
};

impl<'a> Lexer<'a> {
  fn consume_next(&mut self, token: Token) -> Result<(), ()> {
    if let Some(t) = self.next() {
      if let Token::LexicalError(err) = t.to_owned() {
        eprintln!("{}", err);
        return Err(());
      }
      if t != token {
        self.panic_compile_error(
          CompileError::syntax_error_template(),
          format!("Expected `{}`, but got `{}`", token, t),
        );
      }
      Ok(())
    } else {
      self.panic_compile_error(
        CompileError::syntax_error_template(),
        format!("Expected `{}`, but got `None`", token),
      );
    }
  }

  fn match_next(&mut self, token: Token) -> Result<bool, bool> {
    if let Some(t) = self.peek() {
      if let Token::LexicalError(err) = t.to_owned() {
        eprintln!("{}", err);
        Err(false)
      } else {
        Ok(*t == token)
      }
    } else {
      self.panic_compile_error(
        CompileError::syntax_error_template(),
        format!("Expected `{}`, but got `None`", token),
      );
    }
  }

  fn consume_next_identifier(&mut self) -> Result<Result<String, ()>, Result<String, ()>> {
    if let Some(t) = self.next() {
      if let Token::LexicalError(err) = t {
        eprintln!("{}", err);
        Err(Err(()))
      } else if let Token::Identifier(id) = t {
        Ok(Ok(id.to_owned()))
      } else {
        Ok(Err(()))
      }
    } else {
      self.panic_compile_error(
        CompileError::syntax_error_template(),
        "Expected <id>, but got `None`".to_string(),
      );
    }
  }

  fn consume_next_integer(&mut self) -> Result<Result<i64, ()>, Result<i64, ()>> {
    if let Some(t) = self.next() {
      if let Token::LexicalError(err) = t {
        eprintln!("{}", err);
        Err(Err(()))
      } else if let Token::Integer(num) = t {
        Ok(Ok(num))
      } else {
        Ok(Err(()))
      }
    } else {
      self.panic_compile_error(
        CompileError::syntax_error_template(),
        "Expected <integer>, but got `None`".to_string(),
      );
    }
  }
}

#[derive(Debug)]
pub struct Parser<'a> {
  lexer: Lexer<'a>,
  ast_entry: Option<Box<ProgramExpr>>,
  has_error: bool,
}

impl<'a> Parser<'a> {
  fn consume_next(&mut self, token: Token) {
    match self.lexer.consume_next(token) {
      Ok(_) => {}
      Err(_) => self.has_error = true,
    }
  }

  fn match_next(&mut self, token: Token) -> bool {
    match self.lexer.match_next(token) {
      Ok(res) => res,
      Err(res) => {
        self.has_error = true;
        res
      }
    }
  }

  fn consume_next_identifier(&mut self) -> Result<String, ()> {
    match self.lexer.consume_next_identifier() {
      Ok(res) => res,
      Err(res) => {
        self.has_error = true;
        res
      }
    }
  }

  fn consume_next_integer(&mut self) -> Result<i64, ()> {
    match self.lexer.consume_next_integer() {
      Ok(res) => res,
      Err(res) => {
        self.has_error = true;
        res
      }
    }
  }
}

impl<'a> Parser<'a> {
  pub fn new(ctx: &'a str) -> Self {
    Self {
      lexer: Lexer::new(ctx),
      ast_entry: None,
      has_error: false,
    }
  }

  pub fn parse(&mut self) -> &mut Self {
    let program_expr = self.parse_program();
    self.ast_entry = Some(program_expr.into());
    if self.has_error {
      panic!("Errors above occurred (during `parsing`), compiling stopped ...")
    }
    self
  }

  pub fn show_ast(&mut self) -> &mut Self {
    println!("AST:");
    println!("{}", SEP.as_str());
    match &self.ast_entry {
      Some(ctx) => println!("{:#?}", ctx),
      None => println!("None"),
    }
    println!("{}", SEP.as_str());
    self
  }
}
