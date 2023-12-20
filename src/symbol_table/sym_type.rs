#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SymType {
  #[default]
  Nil = 0,
  Const = 1,
  Var = 2,
  Proc = 3,
}
