use std::fmt::Display;

use super::traits::ErrorTrait;

#[derive(Debug, Clone, Copy)]
pub enum RuntimeErrorType {}

impl Display for RuntimeErrorType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[derive(Debug, Clone)]
pub struct RuntimeError {
  pub line: usize,
  pub info: String,
  pub error_type: RuntimeErrorType,
}

impl Display for RuntimeError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_string())
  }
}

impl ErrorTrait for RuntimeError {
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
