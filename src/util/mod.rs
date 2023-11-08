use std::collections::HashMap;

use once_cell::sync::Lazy;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VN {
  Program,
  Block,
  ConstDecl,
  Const,
  VarDecl,
  Proc,
  Body,
  Statement,
  LExp,
  Exp,
  Term,
  Factor,
  Lop,
  Aop,
  Mop,
  Id,
  Integer,
}

/// BasicSyntaxUnit
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BasicField {
  VN(VN),
  VT(String),
}

pub type BF = BasicField;

impl From<&str> for BasicField {
  fn from(value: &str) -> Self {
    Self::VT(value.to_string())
  }
}

/// SyntaxUnit
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Field {
  /// Optional
  O(BasicField),
  /// Required
  R(BasicField),
}

pub type F = Field;
pub type Candidate = Vec<Field>;
pub type C = Candidate;

#[allow(unused)]
pub static BNF: Lazy<HashMap<VN, Vec<C>>> = Lazy::new(|| {
  let bnf = vec![];
  unimplemented!();
  bnf.into_iter().collect()
});
