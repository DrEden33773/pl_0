use crate::bytecode::basic::TraceableByteCode;

pub struct Chunk {
  pub(crate) bytecodes: Vec<TraceableByteCode>,
  pub(crate) ip: usize,
  pub(crate) next: usize,
}

pub struct Stack {
  pub(crate) data: Vec<i64>,
  pub(crate) sp: usize,
  pub(crate) top: usize,
}

pub struct VM {
  pub(crate) chunk: Chunk,
  pub(crate) stack: Stack,
}
