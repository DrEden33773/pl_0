use std::fmt::Display;

use crate::error::compile_error::CompileError;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Token {
  /* keywords */
  If,
  Then,
  Else,
  While,
  Do,
  Const,
  Var,
  Procedure,
  Program,
  Begin,
  End,
  Call,
  Read,
  Write,
  Odd,
  /* symbols */
  Add,       // +
  Sub,       // -
  Mul,       // *
  Div,       // /
  Eq,        // =
  Lt,        // <
  Gt,        // >
  Le,        // <=
  Ge,        // >=
  Ne,        // <>
  EqSign,    // :=
  ParL,      // (
  ParR,      // )
  Semicolon, // ;
  Comma,     // ,
  Dot,       // .
  /* Identifier */
  Identifier(String),
  /* constant values */
  Integer(i64),
  /* EOS */
  Eos,
  /* Error */
  LexicalError(CompileError),
}

impl Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::LexicalError(_) => write!(f, "LexicalErrorToken"),
      _ => write!(f, "{:?}", self),
    }
  }
}

impl From<CompileError> for Token {
  fn from(err: CompileError) -> Self {
    Self::LexicalError(err)
  }
}
