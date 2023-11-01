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
      Self::Add => write!(f, "+"),
      Self::Sub => write!(f, "-"),
      Self::Mul => write!(f, "*"),
      Self::Div => write!(f, "/"),
      Self::Eq => write!(f, "="),
      Self::Lt => write!(f, "<"),
      Self::Gt => write!(f, ">"),
      Self::Le => write!(f, "<="),
      Self::Ge => write!(f, ">="),
      Self::Ne => write!(f, "<>"),
      Self::EqSign => write!(f, ":="),
      Self::ParL => write!(f, "("),
      Self::ParR => write!(f, ")"),
      Self::Semicolon => write!(f, ";"),
      Self::Comma => write!(f, ","),
      _ => write!(f, "{:?}", self),
    }
  }
}

impl From<CompileError> for Token {
  fn from(err: CompileError) -> Self {
    Self::LexicalError(err)
  }
}
