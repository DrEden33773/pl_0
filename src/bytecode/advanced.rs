#[derive(Debug, Clone, Copy)]
pub enum ByteCode {}

#[derive(Debug, Clone, Copy)]
pub struct TraceableByteCode {
  pub(super) bytecode: ByteCode,
  pub(super) line_num: usize,
  pub(super) col_num: usize,
}
