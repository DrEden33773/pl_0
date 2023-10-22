#![allow(dead_code)]

pub(super) type ParseResult = ();

use crate::{
  error::compile_error::CompileError,
  lexer::{token_def::Token, Lexer, LexerIterator},
};

impl<'a> Lexer<'a> {
  fn match_next(&mut self, token: Token) {
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

  fn try_match_next(&mut self, token: Token) -> bool {
    if let Some(t) = self.next() {
      t == token
    } else {
      self.panic_compile_error(
        CompileError::syntax_error_template(),
        format!("Expected `{:?}`, but got `None`", token),
      );
    }
  }

  fn observe_next(&mut self, token: Token) {
    if let Some(t) = self.peek() {
      if *t != token {
        let unexpected_token = t.to_owned();
        {
          self.panic_compile_error(
            CompileError::syntax_error_template(),
            format!("Expected `{:?}`, but got `{:?}`", token, unexpected_token),
          );
        }
      }
    } else {
      self.panic_compile_error(
        CompileError::syntax_error_template(),
        format!("Expected `{:?}`, but got `None`", token),
      );
    }
  }

  fn try_observe_next(&mut self, token: Token) -> bool {
    if let Some(t) = self.peek() {
      *t == token
    } else {
      self.panic_compile_error(
        CompileError::syntax_error_template(),
        format!("Expected `{:?}`, but got `None`", token),
      );
    }
  }
}

#[derive(Debug)]
pub struct Parser<'a> {
  lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
  fn parse_program(&mut self) -> ParseResult {
    self.lexer.match_next(Token::Program);
    self.parse_id();
    self.lexer.match_next(Token::Semicolon);
    self.parse_block();
  }

  fn parse_id(&mut self) -> ParseResult {
    if matches!(self.lexer.next(), Some(Token::Identifier(_))) {
    } else {
      self.lexer.panic_compile_error(
        CompileError::syntax_error_template(),
        "Expected <id> token, but not found!".to_string(),
      )
    }
  }

  fn parse_integer(&mut self) -> ParseResult {
    if matches!(self.lexer.next(), Some(Token::Integer(_))) {
    } else {
      self.lexer.panic_compile_error(
        CompileError::syntax_error_template(),
        "Expected <integer> token, but not found!".to_string(),
      )
    }
  }

  fn parse_block(&mut self) -> ParseResult {
    if self.lexer.try_observe_next(Token::Const) {
      self.parse_const_decl();
    }
    if self.lexer.try_observe_next(Token::Var) {
      self.parse_var_decl();
    }
    if self.lexer.try_observe_next(Token::Procedure) {
      self.parse_proc();
    }
    self.parse_body();
  }

  fn parse_const_decl(&mut self) -> ParseResult {
    self.parse_id();
    self.lexer.match_next(Token::EqSign);
    self.parse_integer();
  }

  fn parse_var_decl(&mut self) -> ParseResult {
    self.lexer.match_next(Token::Var);
    self.parse_id();
    while self.lexer.try_observe_next(Token::Comma) {
      self.lexer.match_next(Token::Comma);
      self.parse_id();
    }
  }

  fn parse_proc(&mut self) -> ParseResult {
    self.lexer.match_next(Token::Procedure);
    self.parse_id();
    self.lexer.match_next(Token::ParL);
    if !self.lexer.try_observe_next(Token::ParR) {
      self.parse_id();
      while self.lexer.try_observe_next(Token::Comma) {
        self.lexer.match_next(Token::Comma);
        self.parse_id();
      }
    }
    self.lexer.match_next(Token::ParR);
    self.lexer.match_next(Token::Semicolon);
    self.parse_block();
    while self.lexer.try_observe_next(Token::Semicolon) {
      self.lexer.match_next(Token::Semicolon);
      self.parse_proc();
    }
  }

  fn parse_body(&mut self) -> ParseResult {
    self.lexer.match_next(Token::Begin);
    self.parse_statement();
    if self.lexer.try_observe_next(Token::Semicolon) {
      self.lexer.match_next(Token::Semicolon);
      self.parse_statement();
    }
    self.lexer.match_next(Token::End);
  }

  fn parse_statement(&mut self) -> ParseResult {
    match self.lexer.peek() {
      Some(token) => match token {
        Token::If => {
          self.lexer.match_next(Token::If);
          self.parse_left_exp();
          self.lexer.match_next(Token::Then);
          self.parse_statement();
          if self.lexer.try_observe_next(Token::Else) {
            self.lexer.match_next(Token::Else);
            self.parse_statement();
          }
        }
        Token::While => {
          self.lexer.match_next(Token::While);
          self.parse_left_exp();
          self.lexer.match_next(Token::Do);
          self.parse_statement();
        }
        Token::Call => {
          self.lexer.match_next(Token::Call);
          self.parse_id();
          self.lexer.match_next(Token::ParL);
          if !self.lexer.try_observe_next(Token::ParR) {
            self.parse_exp();
            while self.lexer.try_observe_next(Token::Comma) {
              self.lexer.match_next(Token::Comma);
              self.parse_exp();
            }
          }
          self.lexer.match_next(Token::ParR);
        }
        Token::Read => {
          self.lexer.match_next(Token::Read);
          self.lexer.match_next(Token::ParL);
          self.parse_id();
          while self.lexer.try_observe_next(Token::Comma) {
            self.lexer.match_next(Token::Comma);
            self.parse_id();
          }
          self.lexer.match_next(Token::ParR);
        }
        Token::Write => {
          self.lexer.match_next(Token::Write);
          self.lexer.match_next(Token::ParL);
          self.parse_exp();
          while self.lexer.try_observe_next(Token::Comma) {
            self.lexer.match_next(Token::Comma);
            self.parse_exp();
          }
          self.lexer.match_next(Token::ParR);
        }
        Token::Begin => self.parse_body(),
        Token::Identifier(_) => {
          self.parse_id();
          self.lexer.match_next(Token::EqSign);
          self.parse_exp();
        }
        _ => {
          let unexpected_token = token.to_owned();
          {
            self.lexer.panic_compile_error(
              CompileError::syntax_error_template(),
              format!(
                "Expected a <statement> syntax_unit, but got an illegal token `{:?}`",
                unexpected_token
              ),
            );
          }
        }
      },
      None => self.lexer.panic_compile_error(
        CompileError::syntax_error_template(),
        "Expected a <statement> syntax_unit, but got `None`".to_string(),
      ),
    }
  }

  fn parse_left_exp(&mut self) -> ParseResult {
    if self.lexer.try_observe_next(Token::Odd) {
      self.lexer.match_next(Token::Odd);
      self.parse_exp();
    } else {
      self.parse_exp();
      self.parse_lop();
      self.parse_exp();
    }
  }

  fn parse_exp(&mut self) -> ParseResult {
    let next_token_is_add = { self.lexer.try_observe_next(Token::Add) };
    let next_token_is_sub = { self.lexer.try_observe_next(Token::Sub) };
    if next_token_is_add || next_token_is_sub {
      if next_token_is_add {
        self.lexer.match_next(Token::Add);
      } else {
        self.lexer.match_next(Token::Sub);
      }
    }
    self.parse_term();
  }

  fn parse_term(&mut self) -> ParseResult {
    self.parse_factor();
    while self.lexer.try_observe_next(Token::Mul) || self.lexer.try_observe_next(Token::Div) {
      self.parse_mop();
      self.parse_factor();
    }
  }

  fn parse_factor(&mut self) -> ParseResult {
    if self.lexer.try_observe_next(Token::ParL) {
      self.lexer.match_next(Token::ParL);
      self.parse_exp();
      self.lexer.match_next(Token::ParR);
    } else if matches!(self.lexer.peek(), Some(Token::Identifier(_))) {
      self.parse_id();
    } else if matches!(self.lexer.peek(), Some(Token::Integer(_))) {
      self.parse_integer();
    }
  }

  fn parse_lop(&mut self) -> ParseResult {
    match self.lexer.next() {
      Some(token) => match token {
        Token::Eq => { /* TODO */ }
        Token::Lt => { /* TODO */ }
        Token::Gt => { /* TODO */ }
        Token::Le => { /* TODO */ }
        Token::Ge => { /* TODO */ }
        Token::Ne => { /* TODO */ }
        _ => {
          let unexpected_token = token.to_owned();
          {
            self.lexer.panic_compile_error(
              CompileError::syntax_error_template(),
              format!(
                "Expected a <lop> syntax_unit, but got an illegal token `{:?}`",
                unexpected_token
              ),
            );
          }
        }
      },
      None => {
        self.lexer.panic_compile_error(
          CompileError::syntax_error_template(),
          "Expected a <statement> syntax_unit, but got an illegal token `None`".to_string(),
        );
      }
    }
  }

  fn parse_aop(&mut self) -> ParseResult {
    if self.lexer.try_observe_next(Token::Add) {
      self.lexer.match_next(Token::Add);
    } else if self.lexer.try_observe_next(Token::Sub) {
      self.lexer.match_next(Token::Sub);
    } else {
      self.lexer.panic_compile_error(
        CompileError::syntax_error_template(),
        "Expected an <aop> syntax_unit, but got `None`".to_string(),
      );
    }
  }

  fn parse_mop(&mut self) -> ParseResult {
    if self.lexer.try_observe_next(Token::Mul) {
      self.lexer.match_next(Token::Mul);
    } else if self.lexer.try_observe_next(Token::Div) {
      self.lexer.match_next(Token::Div);
    } else {
      self.lexer.panic_compile_error(
        CompileError::syntax_error_template(),
        "Expected an <mop> syntax_unit, but got `None`".to_string(),
      );
    }
  }
}

impl<'a> Parser<'a> {
  pub fn new(ctx: &'a str) -> Self {
    Self {
      lexer: Lexer::new(ctx),
    }
  }
}
