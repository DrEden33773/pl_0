#[derive(Debug, Clone, Copy)]
pub enum ByteCode {}

#[derive(Debug, Clone, Copy)]
pub struct TraceableByteCode {
  pub(crate) bytecode: ByteCode,
  pub(crate) line_num: usize,
  pub(crate) col_num: usize,
}
