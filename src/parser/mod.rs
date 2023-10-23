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

  fn observe_next(&mut self, token: Token) -> bool {
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
  /// ```bnf
  /// <prog> ::= program <id> ; <block>
  fn parse_program(&mut self) -> ParseResult {
    self.lexer.match_next(Token::Program);
    self.parse_id();
    self.lexer.match_next(Token::Semicolon);
    self.parse_block();
  }

  /// ```bnf
  /// <id> -> @letter { @letter | @digit }
  fn parse_id(&mut self) -> ParseResult {
    if matches!(self.lexer.next(), Some(Token::Identifier(_))) {
    } else {
      self.lexer.panic_compile_error(
        CompileError::syntax_error_template(),
        "Expected <id> token, but not found!".to_string(),
      )
    }
  }

  /// ```bnf
  /// <integer> -> @digit { @digit }
  fn parse_integer(&mut self) -> ParseResult {
    if matches!(self.lexer.next(), Some(Token::Integer(_))) {
    } else {
      self.lexer.panic_compile_error(
        CompileError::syntax_error_template(),
        "Expected <integer> token, but not found!".to_string(),
      )
    }
  }

  /// ```bnf
  /// <block> ::= [<const-decl>][<var-decl>][<proc>]<body>
  fn parse_block(&mut self) -> ParseResult {
    // [<const-decl>]
    if self.lexer.observe_next(Token::Const) {
      self.parse_const_decl();
    }
    // [<var-decl>]
    if self.lexer.observe_next(Token::Var) {
      self.parse_var_decl();
    }
    // [<proc>]
    if self.lexer.observe_next(Token::Procedure) {
      self.parse_proc();
    }
    self.parse_body();
  }

  /// ```bnf
  /// <const-decl> ::= const <const> {, <const>} ;
  fn parse_const_decl(&mut self) -> ParseResult {
    self.lexer.match_next(Token::Const);
    self.parse_const();
    // {, <const>}
    while self.lexer.observe_next(Token::Comma) {
      self.lexer.match_next(Token::Comma);
      self.parse_const();
    }
  }

  /// ```bnf
  /// <const> ::= <id> := <integer>
  fn parse_const(&mut self) -> ParseResult {
    self.parse_id();
    self.lexer.match_next(Token::EqSign);
    self.parse_integer();
  }

  /// ```bnf
  /// <var-decl> ::= var <id> {, <id>} ;
  fn parse_var_decl(&mut self) -> ParseResult {
    self.lexer.match_next(Token::Var);
    self.parse_id();
    // {, <id>}
    while self.lexer.observe_next(Token::Comma) {
      self.lexer.match_next(Token::Comma);
      self.parse_id();
    }
  }

  /// ```bnf
  /// <proc> ::= procedure <id> ( [<id> {, <id>}] ) ; <block> {; <proc>}
  fn parse_proc(&mut self) -> ParseResult {
    self.lexer.match_next(Token::Procedure);
    self.parse_id();
    self.lexer.match_next(Token::ParL);
    // [<id>]
    if !self.lexer.observe_next(Token::ParR) {
      self.parse_id();
      // [<id> {, <id>}]
      while self.lexer.observe_next(Token::Comma) {
        self.lexer.match_next(Token::Comma);
        self.parse_id();
      }
    }
    self.lexer.match_next(Token::ParR);
    self.lexer.match_next(Token::Semicolon);
    self.parse_block();
    // {; <proc>}
    while self.lexer.observe_next(Token::Semicolon) {
      self.lexer.match_next(Token::Semicolon);
      self.parse_proc();
    }
  }

  /// ```bnf
  /// <body> ::= begin <statement> {; <statement>} end
  fn parse_body(&mut self) -> ParseResult {
    self.lexer.match_next(Token::Begin);
    self.parse_statement();
    // {; <statement>}
    while self.lexer.observe_next(Token::Semicolon) {
      self.lexer.match_next(Token::Semicolon);
      self.parse_statement();
    }
    self.lexer.match_next(Token::End);
  }

  /// ```bnf
  /// <statement> ::= <id> := <exp>
  ///               | if <l-exp> then <statement> [else <statement>]
  ///               | while <l-exp> do <statement>
  ///               | call <id> [ ( <exp> {, <exp>} ) ]
  ///               | read ( <id> {, <id>} )
  ///               | write ( <exp> {, <exp>} )
  ///               | <body>
  ///               | read ( <id> {, <id>} )
  ///               | write ( <exp> {, <exp>} )
  fn parse_statement(&mut self) -> ParseResult {
    match self.lexer.peek() {
      Some(token) => match token {
        Token::If => {
          self.lexer.match_next(Token::If);
          self.parse_left_exp();
          self.lexer.match_next(Token::Then);
          self.parse_statement();
          if self.lexer.observe_next(Token::Else) {
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
          // [ ( <exp> ) ]
          if !self.lexer.observe_next(Token::ParR) {
            self.parse_exp();
            // [ ( <exp> {, <exp>} ) ]
            while self.lexer.observe_next(Token::Comma) {
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
          // ( <id> {, <id>} )
          while self.lexer.observe_next(Token::Comma) {
            self.lexer.match_next(Token::Comma);
            self.parse_id();
          }
          self.lexer.match_next(Token::ParR);
        }
        Token::Write => {
          self.lexer.match_next(Token::Write);
          self.lexer.match_next(Token::ParL);
          self.parse_exp();
          // ( <exp> {, <exp>} )
          while self.lexer.observe_next(Token::Comma) {
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

  /// ```bnf
  /// <l-exp> ::= <exp> <lop> <exp> | odd <exp>
  fn parse_left_exp(&mut self) -> ParseResult {
    if self.lexer.observe_next(Token::Odd) {
      self.lexer.match_next(Token::Odd);
      self.parse_exp();
    } else {
      self.parse_exp();
      self.parse_lop();
      self.parse_exp();
    }
  }

  /// ```bnf
  /// <exp> ::= [+|-] <term> {<aop> <term>}
  fn parse_exp(&mut self) -> ParseResult {
    let next_token_is_add = { self.lexer.observe_next(Token::Add) };
    let next_token_is_sub = { self.lexer.observe_next(Token::Sub) };
    if next_token_is_add || next_token_is_sub {
      if next_token_is_add {
        self.lexer.match_next(Token::Add);
      } else {
        self.lexer.match_next(Token::Sub);
      }
    }
    self.parse_term();
  }

  /// ```bnf
  /// <term> ::= <factor> {<mop> <factor>}
  fn parse_term(&mut self) -> ParseResult {
    self.parse_factor();
    while self.lexer.observe_next(Token::Mul) || self.lexer.observe_next(Token::Div) {
      self.parse_mop();
      self.parse_factor();
    }
  }

  /// ```bnf
  /// <factor> ::= <id> | <integer> | (<exp>)
  fn parse_factor(&mut self) -> ParseResult {
    if self.lexer.observe_next(Token::ParL) {
      self.lexer.match_next(Token::ParL);
      self.parse_exp();
      self.lexer.match_next(Token::ParR);
    } else if matches!(self.lexer.peek(), Some(Token::Identifier(_))) {
      self.parse_id();
    } else if matches!(self.lexer.peek(), Some(Token::Integer(_))) {
      self.parse_integer();
    }
  }

  /// ```bnf
  /// <lop> ::= = | <> | < | <= | > | >=
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

  /// ```bnf
  /// <aop> ::= + | -
  fn parse_aop(&mut self) -> ParseResult {
    if self.lexer.observe_next(Token::Add) {
      self.lexer.match_next(Token::Add);
    } else if self.lexer.observe_next(Token::Sub) {
      self.lexer.match_next(Token::Sub);
    } else {
      self.lexer.panic_compile_error(
        CompileError::syntax_error_template(),
        "Expected an <aop> syntax_unit, but got `None`".to_string(),
      );
    }
  }

  /// ```bnf
  /// <mop> ::= * | /
  fn parse_mop(&mut self) -> ParseResult {
    if self.lexer.observe_next(Token::Mul) {
      self.lexer.match_next(Token::Mul);
    } else if self.lexer.observe_next(Token::Div) {
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
