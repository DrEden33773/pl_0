use std::fmt::Display;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SymType {
  #[default]
  Nil = 0,
  Const = 1,
  Var = 2,
  Proc = 3,
}

impl Display for SymType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      SymType::Nil => write!(f, "nil"),
      SymType::Const => write!(f, "const"),
      SymType::Var => write!(f, "var"),
      SymType::Proc => write!(f, "proc"),
    }
  }
}
