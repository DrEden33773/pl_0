use crate::lexer::token_def::Token;

pub trait Expr {}

#[derive(Debug)]
pub struct ProgramExpr {
  pub program: Box<Token>,
  pub id: Box<Token>,
  pub semicolon: Box<Token>,
  pub block: Box<BlockExpr>,
}

#[derive(Debug)]
pub struct BlockExpr {
  pub const_decl: Option<Box<ConstDeclExpr>>,
  pub var_decl: Option<Box<VarDeclExpr>>,
  pub proc: Option<Box<ProcExpr>>,
  pub body: Box<BodyExpr>,
}

#[derive(Debug)]
pub struct ConstDeclExpr {
  pub constants: Vec<Box<ConstExpr>>,
}

#[derive(Debug)]
pub struct ConstExpr {
  pub id: Box<IdExpr>,
  pub integer: Box<IntegerExpr>,
}

#[derive(Debug)]
pub struct VarDeclExpr {
  pub ids: Vec<Box<IdExpr>>,
}

#[derive(Debug)]
pub struct ProcExpr {
  pub id: Box<IdExpr>,
  pub args: Vec<Box<IdExpr>>,
  pub block: Box<BlockExpr>,
  pub procs: Vec<Box<ProcExpr>>,
}

#[derive(Debug)]
pub struct BodyExpr {
  pub statements: Vec<Box<StatementExpr>>,
}

#[derive(Debug)]
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
    id: Box<IdExpr>,
  },
  Write {
    exp: Box<ExpExpr>,
  },
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct ExpExpr {
  pub is_negative: bool,
  pub term: Box<TermExpr>,
  pub aop_terms: Vec<(Box<AopExpr>, Box<TermExpr>)>,
}

#[derive(Debug)]
pub struct TermExpr {
  pub factor: Box<FactorExpr>,
  pub mop_factors: Vec<(Box<MopExpr>, Box<FactorExpr>)>,
}

#[derive(Debug)]
pub enum FactorExpr {
  Id(Box<IdExpr>),
  Integer(Box<IntegerExpr>),
  Exp(Box<ExpExpr>),
}

#[derive(Debug)]
pub enum LopExpr {
  Eq,
  Ne,
  Lt,
  Le,
  Gt,
  Ge,
}

#[derive(Debug)]
pub enum AopExpr {
  Add,
  Sub,
}

#[derive(Debug)]
pub enum MopExpr {
  Mul,
  Div,
}

#[derive(Debug)]
pub struct IdExpr(String);

#[derive(Debug)]
pub struct IntegerExpr(i64);
