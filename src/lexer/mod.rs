use self::token_def::Token;
use crate::error::{compile_error::CompileError, PL0Error};
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
  line_num: usize,
  col_num: usize,
  lexical_errors: Vec<PL0Error>,
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

#[allow(dead_code)]
impl<'a> Lexer<'a> {
  #[deprecated]
  fn panic(&self, message: String) -> ! {
    #[cfg(not(feature = "debug"))]
    {
      println!(
        "Error[Line({}), Col({})] :: {}",
        self.line_num, self.col_num, message
      );
      exit(-1)
    }
    #[cfg(feature = "debug")]
    {
      panic!(
        "Error[Line({}), Col({})] :: {}",
        self.line_num, self.col_num, message
      )
    }
  }

  fn sync_to_curr_token_last_char(&mut self) {
    loop {
      if self.peek_char().is_whitespace() {
        break;
      } else {
        self.next_char();
      }
    }
  }

  pub(super) fn panic_compile_error(
    &mut self,
    mut error_template: CompileError,
    message: String,
  ) -> ! {
    error_template.line = self.line_num;
    error_template.col = self.col_num;
    error_template.info = message;
    let pl0error: PL0Error = error_template.into();
    self.lexical_errors.push(pl0error.to_owned());
    panic!("{}", pl0error);
  }
}

impl<'a> Lexer<'a> {
  fn ascii_char_handler(&mut self, c: char) -> char {
    if c.is_ascii() {
      c
    } else {
      self.panic_compile_error(
        CompileError::lexical_error_template(),
        format!("'{c}' is not an ASCII character"),
      );
      // '\0'
    }
  }

  #[allow(dead_code)]
  fn read_char(&mut self) -> char {
    match self.next_char() {
      Some(c) => self.ascii_char_handler(c),
      None => '\0',
    }
  }

  fn peek_char(&mut self) -> char {
    match self.source.peek() {
      Some(&c) => self.ascii_char_handler(c),
      None => '\0',
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
    for (ahead, candidate) in ahead_cases.into_iter().zip(candidates) {
      if c == ahead {
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
        // '.' => Some(Token::Dot),
        '<' => Some(self.check_ahead(vec!['=', '>'], vec![Token::Le, Token::Ne], Token::Lt)),
        '>' => Some(self.check_ahead(vec!['='], vec![Token::Ge], Token::Gt)),
        ':' => match self.peek_char() {
          '=' => {
            self.next_char();
            Some(Token::EqSign)
          }
          _ => {
            self.panic_compile_error(
              CompileError::lexical_error_template(),
              format!("'{c}' is an undefined sign, did you mean ':='?"),
            );
            // None
          }
        },
        '0'..='9' => self.lexing_integer(c),
        'a'..='z' | 'A'..='Z' => self.lexing_identifier(c),
        c if !c.is_ascii() => {
          self.panic_compile_error(
            CompileError::lexical_error_template(),
            format!("'{c}' is not an ASCII character"),
          );
          // None
        }
        _ => {
          self.panic_compile_error(
            CompileError::lexical_error_template(),
            format!("'{c}' is an unexpected character"),
          );
          // None
        }
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
      lexical_errors: vec![],
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
