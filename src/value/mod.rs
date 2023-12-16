use std::{
  fmt::{Debug, Display},
  hash::Hash,
  rc::Rc,
};

use crate::vm::PL0Closure;

#[derive(Clone)]
pub enum Value {
  Nil,
  Integer(i64),
  PL0Closure(Rc<PL0Closure>),
}

impl Hash for Value {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    match self {
      Self::Nil => (),
      Self::Integer(i) => i.hash(state),
      Self::PL0Closure(p) => Rc::as_ptr(p).hash(state),
    }
  }
}

impl PartialOrd for Value {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    match (self, other) {
      (Self::Integer(l0), Self::Integer(r0)) => l0.partial_cmp(r0),
      (Self::PL0Closure(l0), Self::PL0Closure(r0)) => Rc::as_ptr(l0).partial_cmp(&Rc::as_ptr(r0)),
      _ => None,
    }
  }
}

impl Eq for Value {}

impl PartialEq for Value {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
      (Self::PL0Closure(l0), Self::PL0Closure(r0)) => Rc::as_ptr(l0) == Rc::as_ptr(r0),
      _ => core::mem::discriminant(self) == core::mem::discriminant(other),
    }
  }
}

impl Display for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Debug for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Nil => write!(f, "Nil"),
      Self::Integer(i) => write!(f, "{i}"),
      Self::PL0Closure(p) => write!(f, "Closure(addr = {:?})", Rc::as_ptr(p)),
    }
  }
}
