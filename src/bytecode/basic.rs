use crate::lexer::Lexer;

#[derive(Debug, Clone, Copy)]
pub enum UnaryOpCode {
  SetPositive,
  SetNegative,
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOpCode {
  // arithmetic
  Add,
  Sub,
  Mul,
  Div,
  // comparative
  CmpEq,
  CmpNe,
  CmpGt,
  CmpGe,
  CmpLt,
  CmpLe,
  // assign
  Assign,
}

#[derive(Debug, Clone, Copy)]
pub enum OpCode {
  Unary(UnaryOpCode),
  Binary(BinaryOpCode),
}

#[derive(Debug, Clone, Copy)]
pub enum ByteCode {
  LoadConst { constant: i64 },
  Operation { op_code: OpCode },
  LoadVar { layer_dif: usize, offset: usize },
  SetVar { layer_dif: usize, offset: usize },
  Call { layer_dif: usize, offset: usize },
  IncreaseSp { increment: usize },
  JumpTo { offset: usize },
  JumpIf { offset: usize },
  ReadToVar { layer_dif: usize, offset: usize },
  WriteTop,
}

#[derive(Debug, Clone, Copy)]
pub struct TraceableByteCode {
  pub(crate) bytecode: ByteCode,
  pub(crate) line_num: usize,
  pub(crate) col_num: usize,
}

impl TraceableByteCode {
  pub fn new(bytecode: ByteCode, line_num: usize, col_num: usize) -> Self {
    Self {
      bytecode,
      line_num,
      col_num,
    }
  }

  pub fn new_with_lexer_ref(bytecode: ByteCode, lexer_ref: &Lexer) -> Self {
    Self {
      bytecode,
      line_num: lexer_ref.line_num,
      col_num: lexer_ref.col_num,
    }
  }
}
