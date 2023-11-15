pub trait ErrorTrait {
  fn line(&self) -> usize;
  fn col(&self) -> usize;
  fn info(&self) -> String;
  fn error_type(&self) -> String;
  fn as_string(&self) -> String {
    format!(
      "{}{{ Line: {}, Col: {} }}\n  | ~~ {}\n",
      self.error_type(),
      self.line(),
      self.col(),
      self.info()
    )
  }
}
