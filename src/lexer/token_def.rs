#[derive(Debug, Clone)]
pub enum Token {
  /* keywords */
  If,
  Then,
  While,
  Do,
  Const,
  Var,
  Procedure,
  Program,
  Begin,
  End,
  Call,
  Read,
  Write,
  Odd,
  /* symbols */
  Add,       // +
  Sub,       // -
  Mul,       // *
  Div,       // /
  Eq,        // =
  Lt,        // <
  Gt,        // >
  Le,        // <=
  Ge,        // >=
  Ne,        // <>
  EqSign,    // :=
  ParL,      // (
  ParR,      // )
  Semicolon, // ;
  Comma,     // ,
  Dot,       // .
  /* Identifier */
  Identifier(String),
  /* constant values */
  String(String),
  Integer(i64),
  Float(f64),
  /* EOS */
  Eos,
}
