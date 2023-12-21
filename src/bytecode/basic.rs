#[derive(Debug, Clone, Copy)]
pub enum OpCode {
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
}

#[derive(Debug, Clone, Copy)]
pub enum ByteCode {
  LIT,
  OPR,
  LOD,
  STO,
  CAL,
  INT,
  JMP,
  JPC,
  RED,
  WRT,
}
