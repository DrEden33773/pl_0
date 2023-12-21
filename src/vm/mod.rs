#![allow(unused)]

use crate::{codegen::desc::ActivationRecord, value::Value};
use std::{cell::RefCell, rc::Rc};

pub mod advanced;
pub mod basic;
pub mod lib;

#[derive(Debug)]
pub enum UpValue {
  Open(usize),
  Closed(Value),
}

impl UpValue {
  fn get<'a>(&'a self, stack: &'a [Value]) -> &'a Value {
    match self {
      UpValue::Open(i) => &stack[*i],
      UpValue::Closed(v) => v,
    }
  }

  fn set(&mut self, stack: &mut [Value], value: Value) {
    match self {
      UpValue::Open(i) => stack[*i] = value,
      UpValue::Closed(v) => *v = value,
    }
  }
}

#[derive(Debug)]
pub struct PL0Closure {
  /// Current closure's activation_record
  ar: Rc<ActivationRecord>,
  /// Current closure's up_value list
  up_values: Vec<Rc<RefCell<UpValue>>>,
}
