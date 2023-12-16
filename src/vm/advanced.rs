use crate::value::Value;

pub struct VMFrame {
  /// Data stack (hold `i64` data)
  stack: Vec<Value>,
  /// Stack base of current field/function
  base: usize,
}

impl<'a> VMFrame {
  pub fn get_top(&self) -> usize {
    self.stack.len() - self.base
  }

  pub fn get<T: From<&'a Value>>(&'a self, i: usize) -> T {
    (&self.stack[self.base + i - 1]).into()
  }

  pub fn set(&mut self, i: usize, v: impl Into<Value>) {
    self.stack[self.base + i - 1] = v.into();
  }

  pub fn push(&mut self, v: impl Into<Value>) {
    self.stack.push(v.into());
  }
}

impl VMFrame {
  pub fn new() -> Self {
    Self {
      stack: vec![Value::Nil], // unused entry function
      base: 1,                 // always equals to entry function, even not used
    }
  }
}

impl Default for VMFrame {
  fn default() -> Self {
    Self::new()
  }
}
