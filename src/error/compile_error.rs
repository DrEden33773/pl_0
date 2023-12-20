use std::fmt::Display;

use super::traits::ErrorTrait;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompileError {
  pub line: usize,
  pub col: usize,
  pub info: String,
  pub error_type: CompileErrorType,
}

impl CompileError {
  pub fn show(&self) {
    println!("{}", self)
  }

  pub fn panic(&self) {
    panic!("{}", self)
  }
}

#[allow(dead_code)]
impl CompileError {
  pub fn lexical_error_template() -> Self {
    Self {
      line: 1,
      col: 0,
      info: String::new(),
      error_type: CompileErrorType::LexicalError,
    }
  }

  pub fn syntax_error_template() -> Self {
    Self {
      line: 1,
      col: 0,
      info: String::new(),
      error_type: CompileErrorType::SyntaxError,
    }
  }

  pub fn semantic_error_template() -> Self {
    Self {
      line: 1,
      col: 0,
      info: String::new(),
      error_type: CompileErrorType::SemanticError,
    }
  }
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

  fn col(&self) -> usize {
    self.col
  }
}
