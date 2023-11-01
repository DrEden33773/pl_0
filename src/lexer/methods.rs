use crate::error::compile_error::CompileErrorType;

use super::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;

#[allow(dead_code)]
static KEYWORDS: Lazy<HashMap<&str, Token>> = Lazy::new(|| {
  vec![
    ("if", Token::If),
    ("then", Token::Then),
    ("else", Token::Else),
    ("while", Token::While),
    ("do", Token::Do),
    ("const", Token::Const),
    ("var", Token::Var),
    ("procedure", Token::Procedure),
    ("program", Token::Program),
    ("begin", Token::Begin),
    ("end", Token::End),
    ("call", Token::Call),
    ("read", Token::Read),
    ("write", Token::Write),
    ("odd", Token::Odd),
  ]
  .into_iter()
  .collect()
});

impl<'a> Lexer<'a> {
  pub(super) fn lexing_identifier(&mut self, first: char) -> Option<Token> {
    let mut identifier = format!("{first}");
    loop {
      let c = self.peek_char();
      match c.is_err() {
        true => return c.unwrap_err().into(),
        false => (),
      }
      let c = c.unwrap();
      if c.is_alphanumeric() {
        self.next_char();
        identifier.push(c);
      } else if c == '_' {
        // self.panic_compile_error(
        //   CompileError::lexical_error_template(),
        //   "'_' is not supported for identifier declaration".to_string(),
        // );
        return Some(Token::LexicalError(
          CompileErrorBuilder::new()
            .with_line(self.line_num)
            .with_col(self.col_num)
            .with_error_type(CompileErrorType::LexicalError)
            .with_info("'_' is not supported for identifier declaration".to_string())
            .build(),
        ));
      } else {
        break;
      }
    }
    KEYWORDS
      .get(identifier.as_str())
      .cloned()
      .or(Some(Token::Identifier(identifier)))
  }
}

impl<'a> Lexer<'a> {
  pub(super) fn lexing_integer(&mut self, first: char) -> Option<Token> {
    if !first.is_ascii_digit() {
      // self.panic_compile_error(
      //   CompileError::lexical_error_template(),
      //   format!("'{first}' is not a digit"),
      // );
      return Some(Token::LexicalError(
        CompileErrorBuilder::new()
          .with_line(self.line_num)
          .with_col(self.col_num)
          .with_error_type(CompileErrorType::LexicalError)
          .with_info(format!("'{first}' is not a digit"))
          .build(),
      ));
    }
    let mut scanned = first.to_digit(10).unwrap() as i64;
    loop {
      let c = self.peek_char();
      match c.is_err() {
        true => return c.unwrap_err().into(),
        false => (),
      }
      let c = c.unwrap();
      if c.is_ascii_digit() {
        self.next_char();
        scanned = scanned * 10 + c.to_digit(10).unwrap() as i64;
      } else {
        break;
      }
    }
    Some(Token::Integer(scanned))
  }
}
