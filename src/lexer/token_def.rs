#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Token {
  /* keywords */
  If,
  Then,
  Else,
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
  Integer(i64),
  /* EOS */
  Eos,
}
