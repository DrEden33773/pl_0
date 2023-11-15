pub mod methods;
pub mod synchronizer;

use crate::{
  ast::ProgramExpr,
  error::error_builder::CompileErrorBuilder,
  lexer::{token_def::Token, Lexer, LexerIterator},
  parser::synchronizer::tables::TOKEN_FOLLOW_TABLE,
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
    if self.lexer.peek().is_none() {
      self.has_error = true;
      return;
    }

    let t = self.lexer.peek().cloned().unwrap();

    if let Token::LexicalError(err) = t {
      eprintln!("{}", err);
      self.lexer.next();
      self.has_error = true;
    } else if t != token {
      let unexpected_t = t.to_owned();
      let err = CompileErrorBuilder::syntax_error_template()
        .with_lexer_ref(&self.lexer)
        .with_info(format!("Expected `{}`, but got `{}`", token, unexpected_t))
        .build();
      eprintln!("{}", err);
      if !TOKEN_FOLLOW_TABLE.get(&token).unwrap().contains(&t) {
        self.lexer.next();
      }
      self.has_error = true;
    } else {
      self.lexer.next();
    }
  }

  fn match_next(&mut self, token: Token) -> bool {
    if self.lexer.peek().is_none() {
      self.has_error = true;
      return false;
    }

    let t = self.lexer.peek().cloned().unwrap();

    if let Token::LexicalError(_err) = t {
      // eprintln!("{}", _err); // shouldn't show lexical error while simply matching
      self.has_error = true;
      false
    } else {
      t == token
    }
  }

  fn consume_next_identifier(&mut self) -> Result<String, bool> {
    if self.lexer.peek().is_none() {
      self.has_error = true;
      return Err(false);
    }

    let t = self.lexer.peek().cloned().unwrap();

    if let Token::LexicalError(err) = t {
      eprintln!("{}", err);
      self.lexer.next();
      self.has_error = true;

      Err(true)
    } else if let Token::Identifier(id) = t {
      self.lexer.next();

      Ok(id)
    } else {
      self.has_error = true;
      if !TOKEN_FOLLOW_TABLE
        .get(&Token::Identifier(Default::default()))
        .unwrap()
        .contains(&t)
      {
        self.lexer.next();
      }
      Err(false)
    }
  }

  fn consume_next_integer(&mut self) -> Result<i64, bool> {
    if self.lexer.peek().is_none() {
      self.has_error = true;
      return Err(false);
    }

    let t = self.lexer.peek().cloned().unwrap();

    if let Token::LexicalError(err) = t {
      eprintln!("{}", err);
      self.lexer.next();
      self.has_error = true;

      Err(true)
    } else if let Token::Integer(num) = t {
      self.lexer.next();

      Ok(num)
    } else {
      self.has_error = true;
      if !TOKEN_FOLLOW_TABLE
        .get(&Token::Integer(Default::default()))
        .unwrap()
        .contains(&t)
      {
        self.lexer.next();
      }
      Err(false)
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
    if self.has_error {
      panic!("|> Errors above occurred (during `parsing`), compiling stopped ... <|\n");
    }
    self.ast_entry = program_expr;
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
