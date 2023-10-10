pub trait ErrorTrait {
  fn line(&self) -> usize;
  fn info(&self) -> String;
  fn error_type(&self) -> String;
  fn as_string(&self) -> String {
    format!(
      "{} [Line {}] => {}",
      self.error_type(),
      self.line(),
      self.info()
    )
  }
}
