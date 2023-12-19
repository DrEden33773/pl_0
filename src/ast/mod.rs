use crate::lexer::Lexer;
use derive_more::From;

/// - format = (line_number, colon_number)
#[derive(Debug, Clone, Copy)]
pub struct Location(usize, usize);

impl From<&Lexer<'_>> for Location {
  fn from(lexer: &Lexer<'_>) -> Self {
    Self(lexer.line_num, lexer.col_num)
  }
}

#[derive(Debug, Clone, From)]
pub enum AstExpr {
  ProgramExpr(Box<ProgramExpr>),
  BlockExpr(Box<BlockExpr>),
  ConstDeclExpr(Box<ConstDeclExpr>),
  ConstExpr(Box<ConstExpr>),
  VarDeclExpr(Box<VarDeclExpr>),
  ProcExpr(Box<ProcExpr>),
  BodyExpr(Box<BodyExpr>),
  StatementExpr(Box<StatementExpr>),
  LExpExpr(Box<LExpExpr>),
  ExpExpr(Box<ExpExpr>),
  TermExpr(Box<TermExpr>),
  FactorExpr(Box<FactorExpr>),
  LopExpr(Box<LopExpr>),
  AopExpr(Box<AopExpr>),
  MopExpr(Box<MopExpr>),
  IdExpr(Box<IdExpr>),
  IntegerExpr(Box<IntegerExpr>),
}

#[derive(Debug, Clone)]
pub struct ProgramExpr {
  pub id: Box<IdExpr>,
  pub block: Box<BlockExpr>,
}

#[derive(Debug, Clone)]
pub struct BlockExpr {
  pub const_decl: Option<Box<ConstDeclExpr>>,
  pub var_decl: Option<Box<VarDeclExpr>>,
  pub proc: Option<Box<ProcExpr>>,
  pub body: Box<BodyExpr>,
}

#[derive(Debug, Clone)]
pub struct ConstDeclExpr {
  pub constants: Vec<Box<ConstExpr>>,
}

#[derive(Debug, Clone)]
pub struct ConstExpr {
  pub id: Box<IdExpr>,
  pub integer: Box<IntegerExpr>,
}

#[derive(Debug, Clone)]
pub struct VarDeclExpr {
  pub id_list: Vec<Box<IdExpr>>,
}

#[derive(Debug, Clone)]
pub struct ProcExpr {
  pub id: Box<IdExpr>,
  pub args: Vec<Box<IdExpr>>,
  pub block: Box<BlockExpr>,
  pub procs: Vec<Box<ProcExpr>>,
}

#[derive(Debug, Clone)]
pub struct BodyExpr {
  pub statements: Vec<Box<StatementExpr>>,
}

#[derive(Debug, Clone)]
pub enum StatementExpr {
  Id {
    id: Box<IdExpr>,
    exp: Box<ExpExpr>,
  },
  If {
    l_exp: Box<LExpExpr>,
    then_statement: Box<StatementExpr>,
    else_statement: Option<Box<StatementExpr>>,
  },
  While {
    l_exp: Box<LExpExpr>,
    statement: Box<StatementExpr>,
  },
  Call {
    id: Box<IdExpr>,
    args: Vec<Box<ExpExpr>>,
  },
  Body {
    body: Box<BodyExpr>,
  },
  Read {
    id_list: Vec<Box<IdExpr>>,
  },
  Write {
    exps: Vec<Box<ExpExpr>>,
  },
}

#[derive(Debug, Clone)]
pub enum LExpExpr {
  Exp {
    l_exp: Box<ExpExpr>,
    lop: Box<LopExpr>,
    r_exp: Box<ExpExpr>,
  },
  Odd {
    exp: Box<ExpExpr>,
  },
}

#[derive(Debug, Clone)]
pub struct ExpExpr {
  pub is_negative: bool,
  pub term: Box<TermExpr>,
  pub aop_terms: Vec<(Box<AopExpr>, Box<TermExpr>)>,
}

#[derive(Debug, Clone)]
pub struct TermExpr {
  pub factor: Box<FactorExpr>,
  pub mop_factors: Vec<(Box<MopExpr>, Box<FactorExpr>)>,
}

#[derive(Debug, Clone)]
pub enum FactorExpr {
  Id(Box<IdExpr>),
  Integer(Box<IntegerExpr>),
  Exp(Box<ExpExpr>),
}

#[derive(Debug, Clone, Copy)]
pub enum LopExpr {
  Eq(Location),
  Ne(Location),
  Lt(Location),
  Le(Location),
  Gt(Location),
  Ge(Location),
}

#[derive(Debug, Clone, Copy)]
pub enum AopExpr {
  Add(Location),
  Sub(Location),
}

#[derive(Debug, Clone, Copy)]
pub enum MopExpr {
  Mul(Location),
  Div(Location),
}

#[derive(Debug, Clone)]
pub struct IdExpr(pub String, pub Location);

#[derive(Debug, Clone, Copy)]
pub struct IntegerExpr(pub i64, pub Location);
