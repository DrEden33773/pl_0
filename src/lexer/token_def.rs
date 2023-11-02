use std::{fmt::Display, hash::Hash};

use crate::error::compile_error::CompileError;

#[derive(Debug, Clone, PartialOrd, Ord)]
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
  // Eos,
  /* Error */
  LexicalError(CompileError),
}

impl Hash for Token {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    core::mem::discriminant(self).hash(state);
  }
}

impl Eq for Token {}

impl PartialEq for Token {
  fn eq(&self, other: &Self) -> bool {
    /* match (self, other) {
        (Self::Identifier(l0), Self::Identifier(r0)) => l0 == r0,
        (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
        (Self::LexicalError(l0), Self::LexicalError(r0)) => l0 == r0,
        _ => core::mem::discriminant(self) == core::mem::discriminant(other),
    } */
    match (self, other) {
      (Self::Identifier(_), Self::Identifier(_)) => true,
      (Self::Integer(_), Self::Integer(_)) => true,
      (Self::LexicalError(_), Self::LexicalError(_)) => true,
      _ => core::mem::discriminant(self) == core::mem::discriminant(other),
    }
  }
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
