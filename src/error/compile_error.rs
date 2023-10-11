use std::fmt::Display;

use super::traits::ErrorTrait;

#[derive(Debug, Clone, Copy)]
pub enum CompileErrorType {
  LexicalError,
  SyntaxError,
  SemanticError,
}

impl Display for CompileErrorType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let error_type = match self {
      Self::LexicalError => "LexicalError",
      Self::SyntaxError => "SyntaxError",
      Self::SemanticError => "SemanticError",
    };
    write!(f, "{}", error_type)
  }
}

#[derive(Debug, Clone)]
pub struct CompileError {
  pub line: usize,
  pub info: String,
  pub error_type: CompileErrorType,
}

impl Display for CompileError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_string())
  }
}

impl ErrorTrait for CompileError {
  fn line(&self) -> usize {
    self.line
  }

  fn info(&self) -> String {
    self.info.to_owned()
  }

  fn error_type(&self) -> String {
    self.error_type.to_string()
  }
}
