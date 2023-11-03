use self::token_def::Token;
use crate::error::{compile_error::CompileError, error_builder::CompileErrorBuilder};
use std::{iter::Peekable, str::Chars};

#[cfg(not(feature = "debug"))]
use std::process::exit;

pub mod methods;
pub mod token_def;

pub trait LexerIterator {
  type Item;
  fn peek(&mut self) -> Option<&Self::Item>;
}

#[derive(Debug)]
pub struct Lexer<'a> {
  source: Peekable<Chars<'a>>,
  ahead: Option<Token>,
  pub(super) line_num: usize,
  pub(super) col_num: usize,
}

impl<'a> Iterator for Lexer<'a> {
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item> {
    if self.ahead.is_none() {
      self.do_next()
    } else {
      self.ahead.take()
    }
  }
}

impl<'a> LexerIterator for Lexer<'a> {
  type Item = Token;

  fn peek(&mut self) -> Option<&Self::Item> {
    if self.ahead.is_none() {
      self.ahead = self.do_next();
    }
    self.ahead.as_ref()
  }
}

impl<'a> Lexer<'a> {
  #[deprecated]
  #[allow(dead_code)]
  pub(super) fn panic_compile_error(
    &mut self,
    mut error_template: CompileError,
    message: String,
  ) -> ! {
    error_template.line = self.line_num;
    error_template.col = self.col_num;
    error_template.info = message;
    panic!("{}", error_template);
  }
}

impl<'a> Lexer<'a> {
  fn ascii_char_handler(&mut self, c: char) -> Result<char, Token> {
    if c.is_ascii() {
      Ok(c)
    } else {
      self.next_char();
      Err(
        CompileErrorBuilder::lexical_error_template()
          .with_lexer_ref(self)
          .with_info(format!("'{}' is not an ASCII character", c))
          .build()
          .into(),
      )
    }
  }

  #[allow(dead_code)]
  fn ascii_char_handler_without_skipping_lexical_error(&mut self, c: char) -> Result<char, Token> {
    if c.is_ascii() {
      Ok(c)
    } else {
      Err(
        CompileErrorBuilder::lexical_error_template()
          .with_lexer_ref(self)
          .with_info(format!("'{}' is not an ASCII character", c))
          .build()
          .into(),
      )
    }
  }

  #[allow(dead_code)]
  fn read_char(&mut self) -> Result<char, Token> {
    match self.next_char() {
      Some(c) => self.ascii_char_handler(c),
      None => Ok('\0'),
    }
  }

  fn peek_char(&mut self) -> Result<char, Token> {
    match self.source.peek() {
      Some(&c) => self.ascii_char_handler(c),
      None => Ok('\0'),
    }
  }

  fn next_char(&mut self) -> Option<char> {
    let char = self.source.next();
    if char.is_some() {
      self.col_num += 1;
    }
    char
  }
}

impl<'a> Lexer<'a> {
  fn check_ahead(
    &mut self,
    ahead_cases: Vec<char>,
    candidates: Vec<Token>,
    failed: Token,
  ) -> Token {
    let c = self.peek_char();
    if c.as_ref().is_err() {
      return c.unwrap_err();
    }
    for (ahead, candidate) in ahead_cases.into_iter().zip(candidates) {
      if *c.as_ref().unwrap() == ahead {
        self.next_char();
        return candidate;
      }
    }
    failed
  }

  /// Take out the next token.
  fn do_next(&mut self) -> Option<Token> {
    if let Some(c) = self.next_char() {
      match c {
        c if c.is_whitespace() => {
          if c.is_control() {
            self.line_num += 1;
            self.col_num = 0;
          }
          self.do_next()
        }
        '+' => Some(Token::Add),
        '-' => Some(Token::Sub),
        '*' => Some(Token::Mul),
        '/' => Some(Token::Div),
        '(' => Some(Token::ParL),
        ')' => Some(Token::ParR),
        '=' => Some(Token::Eq),
        ';' => Some(Token::Semicolon),
        ',' => Some(Token::Comma),
        '<' => Some(self.check_ahead(vec!['=', '>'], vec![Token::Le, Token::Ne], Token::Lt)),
        '>' => Some(self.check_ahead(vec!['='], vec![Token::Ge], Token::Gt)),
        ':' => match self.peek_char() {
          Ok(c) => match c {
            '=' => {
              self.next_char();
              Some(Token::EqSign)
            }
            _ => Some(Token::LexicalError(
              CompileErrorBuilder::lexical_error_template()
                .with_lexer_ref(self)
                .with_info("':' is an undefined sign, did you mean ':='?".to_string())
                .build(),
            )),
          },
          Err(err_token) => Some(err_token),
        },
        '0'..='9' => self.lexing_integer(c),
        'a'..='z' | 'A'..='Z' => self.lexing_identifier(c),
        c if !c.is_ascii() => Some(Token::LexicalError(
          CompileErrorBuilder::lexical_error_template()
            .with_lexer_ref(self)
            .with_info(format!("'{}' is not an ASCII character", c))
            .build(),
        )),
        _ => Some(Token::LexicalError(
          CompileErrorBuilder::lexical_error_template()
            .with_lexer_ref(self)
            .with_info(format!("'{}' is an unexpected character", c))
            .build(),
        )),
      }
    } else {
      None
    }
  }
}

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Self {
    Self {
      source: input.chars().peekable(),
      ahead: None,
      line_num: 1,
      col_num: 0, // MUST be zero!
    }
  }
}

impl<'a> Lexer<'a> {
  pub fn dbg_one_pass(input: &'a str) -> Vec<Token> {
    let lexer = Self::new(input);
    let mut token_list = vec![];
    lexer.for_each(|token| token_list.push(token));
    token_list
  }
}
