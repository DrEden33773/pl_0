use std::cmp::Ordering;

use crate::value::Value;

pub mod bnf;

pub fn set_vec(vec: &mut Vec<Value>, i: usize, value: Value) {
  match i.cmp(&vec.len()) {
    Ordering::Less => vec[i] = value,
    Ordering::Equal => vec.push(value),
    Ordering::Greater => {
      vec.resize(i, Value::Nil);
      vec.push(value);
    }
  }
}
