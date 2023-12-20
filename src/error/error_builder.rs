use crate::{ast::Location, lexer::Lexer};

use super::compile_error::{CompileError, CompileErrorType};

#[derive(Debug, Clone, Default)]
pub struct CompileErrorBuilder {
  line: Option<usize>,
  col: Option<usize>,
  info: Option<String>,
  error_type: Option<CompileErrorType>,
}

impl From<Location> for CompileErrorBuilder {
  fn from(value: Location) -> Self {
    CompileErrorBuilder::semantic_error_template()
      .with_line(value.0)
      .with_col(value.1)
  }
}

impl CompileErrorBuilder {
  pub fn lexical_error_template() -> Self {
    Self::template(CompileErrorType::LexicalError)
  }

  pub fn syntax_error_template() -> Self {
    Self::template(CompileErrorType::SyntaxError)
  }

  pub fn semantic_error_template() -> Self {
    Self::template(CompileErrorType::SemanticError)
  }

  pub fn template(error_type: CompileErrorType) -> Self {
    Self {
      line: Some(1),
      col: Some(0),
      info: Some(String::new()),
      error_type: Some(error_type),
    }
  }
}

impl CompileErrorBuilder {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn with_line(mut self, line: usize) -> Self {
    self.line = Some(line);
    self
  }

  pub fn with_col(mut self, col: usize) -> Self {
    self.col = Some(col);
    self
  }

  pub fn with_lexer_ref(mut self, lexer: &Lexer) -> Self {
    self.line = Some(lexer.line_num);
    self.col = Some(lexer.col_num);
    self
  }

  pub fn with_info(mut self, info: String) -> Self {
    self.info = Some(info);
    self
  }

  pub fn with_error_type(mut self, error_type: CompileErrorType) -> Self {
    self.error_type = Some(error_type);
    self
  }

  pub fn build(self) -> CompileError {
    CompileError {
      line: self.line.unwrap_or(1),
      col: self.col.unwrap_or(0),
      info: self.info.unwrap_or_default(),
      error_type: self.error_type.unwrap_or(CompileErrorType::LexicalError),
    }
  }
}
