use std::{cell::RefCell, rc::Rc};

use crate::value::{Table, Value};

use super::lib::io::{lib_read, lib_write};

pub struct VMFrame {
  /// Data stack (hold `Value` data)
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

  pub fn set<T: Into<Value>>(&mut self, i: usize, v: T) {
    self.stack[self.base + i - 1] = v.into();
  }

  pub fn push(&mut self, v: impl Into<Value>) {
    self.stack.push(v.into());
  }
}

impl VMFrame {
  pub fn new() -> Self {
    let array = vec![];
    let table = vec![
      ("write".into(), Value::RustFunction(lib_write)),
      ("read".into(), Value::RustFunction(lib_read)),
    ]
    .into_iter()
    .collect();
    let env = Table::new_with(array, table);
    Self {
      stack: vec![().into(), Rc::new(RefCell::new(env)).into()], // unused entry function & `ENV` table
      base: 1, // always equals to entry function, even not used
    }
  }
}

impl Default for VMFrame {
  fn default() -> Self {
    Self::new()
  }
}
