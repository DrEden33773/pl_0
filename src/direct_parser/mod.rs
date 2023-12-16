pub mod bnf_frame;
pub mod desc;
pub mod detailed_method;

use self::desc::{ActivationRecord, GotoLabel, Level};
use crate::{
  error::error_builder::CompileErrorBuilder,
  lexer::{token_def::Token, Lexer, LexerIterator},
  parser::synchronizer::tables::TOKEN_FOLLOW_TABLE,
};

#[derive(Debug)]
struct ParseContext<'a> {
  levels: Vec<Level>,
  lexer: Lexer<'a>,
}

#[derive(Debug)]
pub struct DirectParser<'a> {
  ctx: ParseContext<'a>,

  /// activation_record
  ar: ActivationRecord,
  /// stack_pointer
  sp: usize,

  goto_list: Vec<GotoLabel>,
  labels: Vec<GotoLabel>,
  has_error: bool,
}

impl<'a> DirectParser<'a> {
  fn consume_next(&mut self, token: Token) {
    if self.ctx.lexer.peek().is_none() {
      self.has_error = true;
      return;
    }

    let t = self.ctx.lexer.peek().cloned().unwrap();

    if let Token::LexicalError(err) = t {
      eprintln!("{}", err);
      self.ctx.lexer.next();
      self.has_error = true;
    } else if t != token {
      let unexpected_t = t.to_owned();
      let err = CompileErrorBuilder::syntax_error_template()
        .with_lexer_ref(&self.ctx.lexer)
        .with_info(format!("Expected `{}`, but got `{}`", token, unexpected_t))
        .build();
      eprintln!("{}", err);
      if !TOKEN_FOLLOW_TABLE.get(&token).unwrap().contains(&t) {
        self.ctx.lexer.next();
      }
      self.has_error = true;
    } else {
      self.ctx.lexer.next();
    }
  }

  fn match_next(&mut self, token: Token) -> bool {
    if self.ctx.lexer.peek().is_none() {
      self.has_error = true;
      return false;
    }

    let t = self.ctx.lexer.peek().cloned().unwrap();

    if let Token::LexicalError(_err) = t {
      // eprintln!("{}", _err); // shouldn't show lexical error while simply matching
      self.has_error = true;
      false
    } else {
      t == token
    }
  }

  fn consume_next_identifier(&mut self) -> Result<String, bool> {
    if self.ctx.lexer.peek().is_none() {
      self.has_error = true;
      return Err(false);
    }

    let t = self.ctx.lexer.peek().cloned().unwrap();

    if let Token::LexicalError(err) = t {
      eprintln!("{}", err);
      self.ctx.lexer.next();
      self.has_error = true;
      Err(true)
    } else if let Token::Identifier(id) = t {
      self.ctx.lexer.next();
      Ok(id)
    } else {
      self.has_error = true;
      if !TOKEN_FOLLOW_TABLE
        .get(&Token::Identifier(Default::default()))
        .unwrap()
        .contains(&t)
      {
        self.ctx.lexer.next();
      }
      Err(false)
    }
  }

  fn consume_next_integer(&mut self) -> Result<i64, bool> {
    if self.ctx.lexer.peek().is_none() {
      self.has_error = true;
      return Err(false);
    }

    let t = self.ctx.lexer.peek().cloned().unwrap();

    if let Token::LexicalError(err) = t {
      eprintln!("{}", err);
      self.ctx.lexer.next();
      self.has_error = true;
      Err(true)
    } else if let Token::Integer(num) = t {
      self.ctx.lexer.next();
      Ok(num)
    } else {
      self.has_error = true;
      if !TOKEN_FOLLOW_TABLE
        .get(&Token::Integer(Default::default()))
        .unwrap()
        .contains(&t)
      {
        self.ctx.lexer.next();
      }
      Err(false)
    }
  }
}
