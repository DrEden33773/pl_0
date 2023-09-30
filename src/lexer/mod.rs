use self::token_def::Token;
use std::{iter::Peekable, process::exit, str::Chars};

pub mod methods;
pub mod token_def;

pub trait LexerIterator {
  type Item;
  fn next(&mut self) -> Option<Self::Item>;
  fn peek(&mut self) -> Option<&Self::Item>;
}

#[derive(Debug)]
pub struct Lexer<'a> {
  source: Peekable<Chars<'a>>,
  ahead: Option<Token>,
  line_num: usize,
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

  fn next(&mut self) -> Option<Self::Item> {
    if self.ahead.is_none() {
      self.do_next()
    } else {
      self.ahead.take()
    }
  }

  fn peek(&mut self) -> Option<&Self::Item> {
    if self.ahead.is_none() {
      self.ahead = self.do_next();
    }
    self.ahead.as_ref()
  }
}

impl<'a> Lexer<'a> {
  fn panic(&self, message: &str) -> ! {
    println!("Error [Line {}] => {}", self.line_num, message);
    exit(-1)
  }
}

impl<'a> Lexer<'a> {
  fn ascii_char_handler(&self, c: char) -> char {
    if c.is_ascii() {
      c
    } else {
      self.panic(format!("'{c}' is not an ASCII character").as_str())
    }
  }

  #[allow(dead_code)]
  fn read_char(&mut self) -> char {
    match self.source.next() {
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
    self.source.next()
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
        ' ' | '\r' | '\n' | '\t' => {
          if matches!(c, '\n') {
            self.line_num += 1;
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
        '.' => Some(Token::Dot),
        '<' => Some(self.check_ahead(vec!['=', '>'], vec![Token::Le, Token::Ne], Token::Lt)),
        '>' => Some(self.check_ahead(vec!['='], vec![Token::Ge], Token::Gt)),
        ':' => match self.peek_char() {
          '=' => {
            self.next_char();
            Some(Token::EqSign)
          }
          _ => self.panic(format!("token '{}' is undefined, did you mean ':='?", c).as_str()),
        },
        '0'..='9' => self.lexing_integer(c),
        'a'..='z' | 'A'..='Z' | '_' => self.lexing_identifier(c),
        _ => self.panic(format!("'{c}' is not an ASCII character").as_str()),
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
