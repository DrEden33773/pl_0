pub mod compile_error;
pub mod runtime_error;
pub mod traits;

use std::{error::Error, fmt::Display};

use self::{compile_error::CompileError, runtime_error::RuntimeError};

#[derive(Debug, Clone)]
pub enum PL0Error {
  CompileError(CompileError),
  RuntimeError(RuntimeError),
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
