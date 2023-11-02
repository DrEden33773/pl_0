pub mod methods;

use crate::{
  ast::ProgramExpr,
  error::{
    compile_error::{CompileError, CompileErrorType},
    error_builder::CompileErrorBuilder,
  },
  lexer::{token_def::Token, Lexer, LexerIterator},
  SEP,
};

#[derive(Debug)]
pub struct Parser<'a> {
  lexer: Lexer<'a>,
  ast_entry: Option<Box<ProgramExpr>>,
  has_error: bool,
  panic_mode: bool,
}

impl<'a> Parser<'a> {
  fn consume_next(&mut self, token: Token) {
    let t = self.lexer.peek().unwrap();

    if let Token::LexicalError(err) = t.to_owned() {
      eprintln!("{}", err);
      self.lexer.next();
      self.has_error = true;
    } else if *t != token {
      let unexpected_t = t.to_owned();
      let err = CompileErrorBuilder::new()
        .with_line(self.lexer.line_num)
        .with_col(self.lexer.col_num)
        .with_error_type(CompileErrorType::SyntaxError)
        .with_info(format!("Expected `{}`, but got `{}`", token, unexpected_t))
        .build();
      eprintln!("{}", err);
      self.has_error = true;
      self.panic_mode = true;
    } else {
      if self.panic_mode {
        self.panic_mode = false;
      }
      self.lexer.next();
    }
  }

  fn match_next(&mut self, token: Token) -> bool {
    let t = self.lexer.peek().unwrap();

    if let Token::LexicalError(err) = t.to_owned() {
      eprintln!("{}", err);
      self.has_error = true;
      false
    } else {
      *t == token
    }
  }

  fn consume_next_identifier(&mut self) -> Result<String, ()> {
    let t = self.lexer.peek().unwrap();

    if let Token::LexicalError(err) = t {
      eprintln!("{}", err);
      self.lexer.next();
      self.has_error = true;
      Err(())
    } else if let Token::Identifier(id) = t {
      let id = id.to_owned();
      self.lexer.next();
      Ok(id)
    } else {
      Err(())
    }
  }

  fn consume_next_integer(&mut self) -> Result<i64, ()> {
    let t = self.lexer.peek().unwrap();

    if let Token::LexicalError(err) = t {
      eprintln!("{}", err);
      self.lexer.next();
      self.has_error = true;
      Err(())
    } else if let Token::Integer(num) = t {
      let num = num.to_owned();
      self.lexer.next();
      Ok(num)
    } else {
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
      panic_mode: false,
    }
  }

  pub fn parse(&mut self) -> &mut Self {
    let program_expr = self.parse_program();
    if self.has_error {
      panic!("|> Errors above occurred (during `parsing`), compiling stopped ... <|\n")
    }
    self.ast_entry = Some(program_expr.into());
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
