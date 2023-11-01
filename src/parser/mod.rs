// #![allow(dead_code)]

mod methods;

use crate::{
  ast::ProgramExpr,
  error::compile_error::CompileError,
  lexer::{token_def::Token, Lexer, LexerIterator},
  SEP,
};

#[derive(Debug)]
pub struct Parser<'a> {
  lexer: Lexer<'a>,
  ast_entry: Option<Box<ProgramExpr>>,
  has_error: bool,
}

impl<'a> Parser<'a> {
  fn consume_next(&mut self, token: Token) {
    if let Some(t) = self.lexer.peek() {
      if let Token::LexicalError(err) = t.to_owned() {
        eprintln!("{}", err);
        self.has_error = true;
      } else if *t != token {
        let unexpected_t = t.to_owned();
        self.lexer.panic_compile_error(
          CompileError::syntax_error_template(),
          format!("Expected `{}`, but got `{}`", token, unexpected_t),
        );
      } else {
        self.lexer.next();
      }
    } else {
      self.has_error = true;
    }
  }

  fn match_next(&mut self, token: Token) -> bool {
    if let Some(t) = self.lexer.peek() {
      if let Token::LexicalError(err) = t.to_owned() {
        eprintln!("{}", err);
        self.has_error = true;
        false
      } else {
        *t == token
      }
    } else {
      self.has_error = true;
      false
    }
  }

  fn consume_next_identifier(&mut self) -> Result<String, ()> {
    if let Some(t) = self.lexer.peek() {
      if let Token::LexicalError(err) = t {
        eprintln!("{}", err);
        self.has_error = true;
        Err(())
      } else if let Token::Identifier(id) = t {
        let id = id.to_owned();
        self.lexer.next();
        Ok(id.to_owned())
      } else {
        Err(())
      }
    } else {
      self.has_error = true;
      Err(())
    }
  }

  fn consume_next_integer(&mut self) -> Result<i64, ()> {
    if let Some(t) = self.lexer.peek() {
      if let Token::LexicalError(err) = t {
        eprintln!("{}", err);
        self.has_error = true;
        Err(())
      } else if let Token::Integer(num) = t {
        let num = num.to_owned();
        self.lexer.next();
        Ok(num)
      } else {
        Err(())
      }
    } else {
      self.has_error = true;
      Err(())
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
