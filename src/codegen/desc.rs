use crate::{bytecode::advanced::ByteCode, error::compile_error::CompileError, value::Value};

pub(super) type FnBc2u8 = fn(u8, u8) -> ByteCode;
pub(super) type FnBc3u8 = fn(u8, u8, u8) -> ByteCode;
pub(super) type FnBcBool = fn(u8, u8, bool) -> ByteCode;

/// Expression description, inner layer between source code and byte code
#[derive(Debug, PartialEq, Clone)]
pub(super) enum ExprDesc {
  // Constants
  Nil,
  Integer(i64),
  Boolean(bool),
  String(String),

  // Variables
  Local(usize),
  UpValue(usize),

  // function Call
  Function(usize),
  Closure(usize),
  Call(usize, usize),
  VarArgs,

  // table index
  Index(usize, usize),
  IndexField(usize, usize),
  IndexInt(usize, u8),
  IndexUpField(usize, usize), // covers global variables

  // Arithmetic Operators
  UnaryOp {
    op: FnBc2u8,
    operand: usize,
  },
  BinaryOp {
    op: FnBc3u8,
    l_operand: usize,
    r_operand: usize,
  },

  // binary logical operators: 'and', 'or'
  Test {
    condition: Box<ExprDesc>,
    true_list: Vec<usize>,
    false_list: Vec<usize>,
  },

  // Relational Operators
  Compare {
    op: FnBcBool,
    l_operand: usize,
    r_operand: usize,
    true_list: Vec<usize>,
    false_list: Vec<usize>,
  },
}

impl From<String> for ExprDesc {
  fn from(v: String) -> Self {
    Self::String(v)
  }
}

impl From<bool> for ExprDesc {
  fn from(v: bool) -> Self {
    Self::Boolean(v)
  }
}

impl From<i64> for ExprDesc {
  fn from(v: i64) -> Self {
    Self::Integer(v)
  }
}

impl From<()> for ExprDesc {
  fn from(_: ()) -> Self {
    Self::Nil
  }
}

// see discharge_const()
#[derive(Debug, Clone)]
pub(super) enum ConstStack {
  Const(usize),
  Stack(usize),
}

/// Index of locals/up_values in upper functions
#[derive(Debug, Clone)]
pub(super) enum UpIndex {
  Local(usize),
  UpValue(usize),
}

/// Activation record for `procedure` (aka. `closure`)
#[derive(Debug, Default, Clone)]
pub struct ActivationRecord {
  pub(super) has_var_args: bool,
  pub(super) n_param: usize,
  pub(super) constants: Vec<Value>,
  pub(super) up_indexes: Vec<UpIndex>,
  pub(super) byte_codes: Vec<ByteCode>,
}

/// Level of inner functions, used for matching up_value
#[derive(Debug, Default, Clone)]
pub(super) struct Level {
  /// (name, referred_as_up_value)
  pub(super) locals: Vec<(String, bool)>,
  /// (name, index_of_up_value)
  pub(super) upvalues: Vec<(String, UpIndex)>,
}

/// Mark both goto and label
#[derive(Debug)]
pub(super) struct GotoLabel {
  pub(super) name: String,
  pub(super) i_code: usize,
  pub(super) n_var: usize,
}
