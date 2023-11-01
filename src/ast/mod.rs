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
  Eq,
  Ne,
  Lt,
  Le,
  Gt,
  Ge,
}

#[derive(Debug, Clone, Copy)]
pub enum AopExpr {
  Add,
  Sub,
}

#[derive(Debug, Clone, Copy)]
pub enum MopExpr {
  Mul,
  Div,
}

#[derive(Debug, Clone)]
pub struct IdExpr(pub String);

#[derive(Debug, Clone)]
pub struct IntegerExpr(pub i64);
