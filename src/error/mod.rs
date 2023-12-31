pub mod compile_error;
pub mod error_builder;
pub mod runtime_error;
pub mod traits;

use self::{compile_error::CompileError, runtime_error::RuntimeError};
use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub enum PL0Error {
  CompileError(CompileError),
  RuntimeError(RuntimeError),
}

impl From<CompileError> for PL0Error {
  fn from(err: CompileError) -> Self {
    Self::CompileError(err)
  }
}

impl From<RuntimeError> for PL0Error {
  fn from(err: RuntimeError) -> Self {
    Self::RuntimeError(err)
  }
}

impl Display for PL0Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::CompileError(e) => write!(f, "CompileError.{}", e),
      Self::RuntimeError(e) => write!(f, "RuntimeError.{}", e),
    }
  }
}

impl Error for PL0Error {}
