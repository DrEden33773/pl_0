#[derive(Debug, Clone, Copy)]
pub enum ByteCode {
  // up_values
  GetUpvalue(u8, u8),
  SetUpvalue(u8, u8),
  SetUpvalueConst(u8, u8),
  Close(u8),

  // condition structures
  Jump(i16),
  TestAndJump(u8, i16),
  TestOrJump(u8, i16),
  TestAndSetJump(u8, u8, u8),
  TestOrSetJump(u8, u8, u8),

  // function call
  Return0,
}
