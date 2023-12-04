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
