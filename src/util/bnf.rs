use crate::lexer::token_def::Token;
use once_cell::sync::Lazy;
use std::collections::HashMap;

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Elem {
  VN(VN),
  VT(Token),
}

impl From<VN> for Elem {
  fn from(val: VN) -> Self {
    Elem::VN(val)
  }
}

impl From<Token> for Elem {
  fn from(val: Token) -> Self {
    Elem::VT(val)
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ExElem {
  pub elem_list: Vec<Elem>,
  pub is_optional: bool,
  pub is_repeated: bool,
}

impl From<ExElemBuilder> for ExElem {
  fn from(builder: ExElemBuilder) -> Self {
    builder.build()
  }
}

#[derive(Default)]
pub struct ExElemBuilder {
  elem_list: Vec<Elem>,
  is_optional: bool,
  is_repeated: bool,
}

impl ExElemBuilder {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn with_elem(mut self, elem: Elem) -> Self {
    self.elem_list.push(elem);
    self
  }

  pub fn with_elem_list(mut self, elem_list: Vec<Elem>) -> Self {
    self.elem_list = elem_list;
    self
  }

  pub fn optional(mut self) -> Self {
    self.is_optional = true;
    self
  }

  pub fn repeated(mut self) -> Self {
    self.is_repeated = true;
    self
  }

  pub fn build(self) -> ExElem {
    ExElem {
      elem_list: self.elem_list,
      is_optional: self.is_optional,
      is_repeated: self.is_repeated,
    }
  }
}

#[allow(unused)]
pub static BNF: Lazy<HashMap<VN, Vec<Vec<ExElem>>>> = Lazy::new(|| {
  unimplemented!();

  let prog_form = vec![vec![
    ExElemBuilder::new().with_elem(Token::Program.into()).into(),
    ExElemBuilder::new().with_elem(VN::Id.into()).into(),
    ExElemBuilder::new()
      .with_elem(Token::Semicolon.into())
      .into(),
    ExElemBuilder::new().with_elem(VN::Block.into()).into(),
  ]];
  let block_form = vec![vec![
    ExElemBuilder::new()
      .with_elem(VN::ConstDecl.into())
      .optional()
      .into(),
    ExElemBuilder::new()
      .with_elem(VN::VarDecl.into())
      .optional()
      .into(),
    ExElemBuilder::new()
      .with_elem(VN::Proc.into())
      .optional()
      .into(),
    ExElemBuilder::new().with_elem(VN::Body.into()).into(),
  ]];
  let const_decl_form = vec![vec![
    ExElemBuilder::new().with_elem(Token::Const.into()).into(),
    ExElemBuilder::new().with_elem(VN::Const.into()).into(),
    ExElemBuilder::new()
      .with_elem_list(vec![Token::Comma.into(), VN::Const.into()])
      .into(),
    ExElemBuilder::new()
      .with_elem(Token::Semicolon.into())
      .into(),
  ]];
  let const_form = vec![vec![
    ExElemBuilder::new().with_elem(VN::Id.into()).into(),
    ExElemBuilder::new().with_elem(Token::EqSign.into()).into(),
    ExElemBuilder::new().with_elem(VN::Integer.into()).into(),
  ]];
  let var_decl_form = vec![vec![
    ExElemBuilder::new().with_elem(Token::Var.into()).into(),
    ExElemBuilder::new().with_elem(VN::Id.into()).into(),
    ExElemBuilder::new()
      .with_elem_list(vec![Token::Comma.into(), VN::Id.into()])
      .into(),
    ExElemBuilder::new()
      .with_elem(Token::Semicolon.into())
      .into(),
  ]];
  let proc_form = vec![vec![
    ExElemBuilder::new()
      .with_elem(Token::Procedure.into())
      .into(),
    ExElemBuilder::new().with_elem(VN::Id.into()).into(),
    ExElemBuilder::new().with_elem(Token::ParL.into()).into(),
    ExElemBuilder::new().with_elem(Token::ParR.into()).into(),
  ]];
  let bnf = vec![
    (VN::Program, prog_form),
    (VN::Block, block_form),
    (VN::ConstDecl, const_decl_form),
    (VN::Const, const_form),
    (VN::VarDecl, var_decl_form),
    (VN::Proc, proc_form),
  ];
  bnf.into_iter().collect()
});
