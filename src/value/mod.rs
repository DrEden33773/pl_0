use std::{
  cell::RefCell,
  collections::HashMap,
  fmt::{Debug, Display},
  hash::Hash,
  rc::Rc,
};

use crate::{
  util::set_vec,
  vm::{advanced::VMFrame, PL0Closure},
};

const SHORT_STR_MAX: usize = 14; // sizeof(Value) - 1(tag) - 1(len)
const MID_STR_MAX: usize = 48 - 1;

pub struct Table {
  pub array: Vec<Value>,
  pub map: HashMap<Value, Value>,
}

impl Table {
  pub fn new(n_array: usize, n_map: usize) -> Self {
    Table {
      array: Vec::with_capacity(n_array),
      map: HashMap::with_capacity(n_map),
    }
  }

  pub fn new_with(array: Vec<Value>, map: HashMap<Value, Value>) -> Self {
    Table { array, map }
  }

  pub fn index_by(&self, key: &Value) -> &Value {
    match key {
      &Value::Integer(i) => self.index_array(i),
      _ => self.map.get(key).unwrap_or(&Value::Nil),
    }
  }

  pub fn index_array(&self, i: i64) -> &Value {
    self
      .array
      .get(i as usize - 1)
      .unwrap_or_else(|| self.map.get(&Value::Integer(i)).unwrap_or(&Value::Nil))
  }

  pub fn new_index(&mut self, key: Value, value: Value) {
    match key {
      Value::Integer(i) => self.new_index_array(i, value),
      _ => {
        self.map.insert(key, value);
      }
    }
  }

  pub fn new_index_array(&mut self, i: i64, value: Value) {
    if i > 0 && (i < 4 || i < self.array.capacity() as i64 * 2) {
      set_vec(&mut self.array, i as usize - 1, value);
    } else {
      self.map.insert(Value::Integer(i), value);
    }
  }
}

#[derive(Clone)]
pub enum Value {
  Nil,
  Integer(i64),
  ShortStr(u8, [u8; SHORT_STR_MAX]),
  MidStr(Rc<(u8, [u8; MID_STR_MAX])>),
  LongStr(Rc<Vec<u8>>),
  Table(Rc<RefCell<Table>>),
  PL0Closure(Rc<PL0Closure>),
  RustFunction(fn(&mut VMFrame) -> i32),
}

impl From<for<'a> fn(&'a mut VMFrame) -> i32> for Value {
  fn from(v: for<'a> fn(&'a mut VMFrame) -> i32) -> Self {
    Self::RustFunction(v)
  }
}

impl From<Rc<RefCell<Table>>> for Value {
  fn from(v: Rc<RefCell<Table>>) -> Self {
    Self::Table(v)
  }
}

impl From<()> for Value {
  fn from(_: ()) -> Self {
    Self::Nil
  }
}

impl From<Rc<PL0Closure>> for Value {
  fn from(v: Rc<PL0Closure>) -> Self {
    Self::PL0Closure(v)
  }
}

impl From<i64> for Value {
  fn from(v: i64) -> Self {
    Self::Integer(v)
  }
}

// convert &[u8], Vec<u8>, &str and String into Value
impl From<&[u8]> for Value {
  fn from(v: &[u8]) -> Self {
    vec_to_short_mid_str(v).unwrap_or_else(|| Value::LongStr(Rc::new(v.to_vec())))
  }
}
impl From<&str> for Value {
  fn from(s: &str) -> Self {
    s.as_bytes().into() // &[u8]
  }
}

impl From<Vec<u8>> for Value {
  fn from(v: Vec<u8>) -> Self {
    vec_to_short_mid_str(&v).unwrap_or_else(|| Value::LongStr(Rc::new(v)))
  }
}
impl From<String> for Value {
  fn from(s: String) -> Self {
    s.into_bytes().into() // Vec<u8>
  }
}
fn vec_to_short_mid_str(v: &[u8]) -> Option<Value> {
  let len = v.len();
  if len <= SHORT_STR_MAX {
    let mut buf = [0; SHORT_STR_MAX];
    buf[..len].copy_from_slice(v);
    Some(Value::ShortStr(len as u8, buf))
  } else if len <= MID_STR_MAX {
    let mut buf = [0; MID_STR_MAX];
    buf[..len].copy_from_slice(v);
    Some(Value::MidStr(Rc::new((len as u8, buf))))
  } else {
    None
  }
}

impl Hash for Value {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    match self {
      Value::Nil => (),
      Value::Integer(i) => i.hash(state),
      Value::PL0Closure(p) => Rc::as_ptr(p).hash(state),
      Value::Table(t) => Rc::as_ptr(t).hash(state),
      Value::RustFunction(f) => std::ptr::addr_of!(f).hash(state),
      Value::ShortStr(len, buf) => buf[..*len as usize].hash(state),
      Value::MidStr(s) => s.1[..s.0 as usize].hash(state),
      Value::LongStr(s) => s.hash(state),
    }
  }
}

impl PartialOrd for Value {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    match (self, other) {
      (Value::Integer(l0), Value::Integer(r0)) => l0.partial_cmp(r0),
      (Value::PL0Closure(l0), Value::PL0Closure(r0)) => Rc::as_ptr(l0).partial_cmp(&Rc::as_ptr(r0)),
      (Value::Table(l0), Value::Table(r0)) => Rc::as_ptr(l0).partial_cmp(&Rc::as_ptr(r0)),

      // strings of same subtype
      (Value::ShortStr(len1, buf1), Value::ShortStr(len2, buf2)) => {
        Some(buf1[..*len1 as usize].cmp(&buf2[..*len2 as usize]))
      }
      (Value::MidStr(s1), Value::MidStr(s2)) => {
        Some(s1.1[..s1.0 as usize].cmp(&s2.1[..s2.0 as usize]))
      }
      (Value::LongStr(s1), Value::LongStr(s2)) => Some(s1.cmp(s2)),

      // strings of different subtype
      (Value::ShortStr(len1, s1), Value::MidStr(s2)) => {
        Some(s1[..*len1 as usize].cmp(&s2.1[..s2.0 as usize]))
      }
      (Value::ShortStr(len1, s1), Value::LongStr(s2)) => Some(s1[..*len1 as usize].cmp(s2)),
      (Value::MidStr(s1), Value::ShortStr(len2, s2)) => {
        Some(s1.1[..s1.0 as usize].cmp(&s2[..*len2 as usize]))
      }
      (Value::MidStr(s1), Value::LongStr(s2)) => Some(s1.1[..s1.0 as usize].cmp(s2)),
      (Value::LongStr(s1), Value::ShortStr(len2, s2)) => {
        Some(s1.as_ref().as_slice().cmp(&s2[..*len2 as usize]))
      }
      (Value::LongStr(s1), Value::MidStr(s2)) => {
        Some(s1.as_ref().as_slice().cmp(&s2.1[..s2.0 as usize]))
      }

      _ => None,
    }
  }
}

impl Eq for Value {}

impl PartialEq for Value {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Value::Integer(l0), Value::Integer(r0)) => l0 == r0,
      (Value::PL0Closure(l0), Value::PL0Closure(r0)) => Rc::as_ptr(l0) == Rc::as_ptr(r0),
      (Value::Table(l0), Value::Table(r0)) => Rc::as_ptr(l0) == Rc::as_ptr(r0),
      (Value::RustFunction(l0), Value::RustFunction(r0)) => std::ptr::eq(l0, r0),
      (Value::ShortStr(len1, buf1), Value::ShortStr(len2, buf2)) => {
        buf1[..*len1 as usize] == buf2[..*len2 as usize]
      }
      (Value::MidStr(s1), Value::MidStr(s2)) => s1.1[..s1.0 as usize] == s2.1[..s2.0 as usize],
      (Value::LongStr(s1), Value::LongStr(s2)) => s1 == s2,
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
      Value::Nil => write!(f, "Nil"),
      Value::Integer(i) => write!(f, "{i}"),
      Value::PL0Closure(p) => write!(f, "Closure(addr = {:?})", Rc::as_ptr(p)),
      Value::Table(t) => write!(f, "Table(addr = {:?})", Rc::as_ptr(t)),
      Value::RustFunction(func) => write!(f, "LibFunction(addr = {:?})", std::ptr::addr_of!(func)),
      Value::ShortStr(len, buf) => {
        write!(f, "'{}'", String::from_utf8_lossy(&buf[..*len as usize]))
      }
      Value::MidStr(s) => write!(f, "\"{}\"", String::from_utf8_lossy(&s.1[..s.0 as usize])),
      Value::LongStr(s) => write!(f, "'''{}'''", String::from_utf8_lossy(s)),
    }
  }
}
