use super::*;

#[allow(dead_code)]
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
pub(crate) enum Field {
  Prog,
  Block,
  ConstDecl,
  Const,
  VarDecl,
  Proc,
  Body,
  Statement,
  LExp,
  Exp,
  Term,
  Factor,
  Lop,
  Aop,
  Mop,
  Id,
  Integer,
}

pub(crate) static FIELD_FOLLOW_TABLE: Lazy<HashMap<Field, HashSet<Token>>> = Lazy::new(|| {
  type Set = HashSet<Token>;
  let prog_follow = Set::default();
  let proc_follow = FIELD_FIRST_TABLE.get(&Field::Body).unwrap().clone();
  let block_follow = {
    let src = vec![Token::Semicolon].into_iter().collect::<Set>();
    let candidates = vec![prog_follow.clone(), proc_follow.clone()];
    candidates
      .into_iter()
      .fold(src, |acc, x| acc.union(&x).cloned().collect())
  };
  let const_decl_follow = {
    let unions = vec![
      FIELD_FIRST_TABLE.get(&Field::VarDecl).unwrap().clone(),
      FIELD_FIRST_TABLE.get(&Field::Proc).unwrap().clone(),
      FIELD_FIRST_TABLE.get(&Field::Body).unwrap().clone(),
    ];
    unions
      .into_iter()
      .fold(Set::default(), |acc, x| acc.union(&x).cloned().collect())
  };
  let const_follow = vec![Token::Semicolon, Token::Comma].into_iter().collect();
  let var_decl_follow = {
    let unions = vec![
      FIELD_FIRST_TABLE.get(&Field::Proc).unwrap().clone(),
      FIELD_FIRST_TABLE.get(&Field::Body).unwrap().clone(),
    ];
    unions
      .into_iter()
      .fold(Set::default(), |acc, x| acc.union(&x).cloned().collect())
  };
  let body_follow = {
    let unions = vec![prog_follow.clone(), proc_follow.clone()];
    unions
      .into_iter()
      .fold(Set::default(), |acc, x| acc.union(&x).cloned().collect())
  };
  let statement_follow = vec![Token::Semicolon, Token::End, Token::Else]
    .into_iter()
    .collect::<Set>();
  let l_exp_follow = vec![Token::Then, Token::Do].into_iter().collect::<Set>();
  let exp_follow = {
    let src = vec![Token::ParR, Token::Comma].into_iter().collect::<Set>();
    let candidates = vec![
      statement_follow.clone(),
      l_exp_follow.clone(),
      FIELD_FIRST_TABLE.get(&Field::Lop).unwrap().clone(),
    ];
    candidates
      .into_iter()
      .fold(src, |acc, x| acc.union(&x).cloned().collect())
  };
  let term_follow = exp_follow
    .clone()
    .union(FIELD_FIRST_TABLE.get(&Field::Aop).unwrap())
    .cloned()
    .collect::<Set>();
  let factor_follow = term_follow
    .clone()
    .union(FIELD_FIRST_TABLE.get(&Field::Mop).unwrap())
    .cloned()
    .collect::<Set>();
  let lop_follow = FIELD_FIRST_TABLE.get(&Field::Exp).unwrap().clone();
  let aop_follow = FIELD_FIRST_TABLE.get(&Field::Term).unwrap().clone();
  let mop_follow = FIELD_FIRST_TABLE.get(&Field::Factor).unwrap().clone();
  let id_follow = vec![
    Token::Semicolon,
    Token::EqSign,
    Token::Comma,
    Token::ParL,
    Token::ParR,
  ]
  .into_iter()
  .collect::<Set>()
  .union(&factor_follow)
  .cloned()
  .collect();
  let integer_follow = factor_follow
    .clone()
    .union(&const_follow)
    .cloned()
    .collect();
  vec![
    (Field::Prog, prog_follow),
    (Field::Block, block_follow),
    (Field::ConstDecl, const_decl_follow),
    (Field::Const, const_follow),
    (Field::VarDecl, var_decl_follow),
    (Field::Proc, proc_follow),
    (Field::Body, body_follow),
    (Field::Statement, statement_follow),
    (Field::LExp, l_exp_follow),
    (Field::Exp, exp_follow),
    (Field::Term, term_follow),
    (Field::Factor, factor_follow),
    (Field::Lop, lop_follow),
    (Field::Aop, aop_follow),
    (Field::Mop, mop_follow),
    (Field::Id, id_follow),
    (Field::Integer, integer_follow),
  ]
  .into_iter()
  .collect()
});

pub(crate) static FIELD_FIRST_TABLE: Lazy<HashMap<Field, HashSet<Token>>> = Lazy::new(|| {
  type Set = HashSet<Token>;
  let id_token: Set = vec![Token::Identifier(Default::default())]
    .into_iter()
    .collect();
  let prog_first: Set = vec![Token::Program].into_iter().collect();
  let const_decl_first: Set = vec![Token::Const].into_iter().collect();
  let var_decl_first: Set = vec![Token::Var].into_iter().collect();
  let proc_first: Set = vec![Token::Procedure].into_iter().collect();
  let body_first: Set = vec![Token::Begin].into_iter().collect();
  let block_first = {
    let candidates = vec![
      const_decl_first.clone(),
      var_decl_first.clone(),
      proc_first.clone(),
    ];
    candidates.into_iter().fold(body_first.clone(), |acc, x| {
      acc.union(&x).cloned().collect()
    })
  };
  let const_first = vec![Token::Identifier(Default::default())]
    .into_iter()
    .collect();
  let statement_first = {
    let src: Set = vec![
      Token::If,
      Token::While,
      Token::Call,
      Token::Read,
      Token::Write,
    ]
    .into_iter()
    .collect();
    let candidates = vec![id_token.clone(), body_first.clone()];
    candidates
      .into_iter()
      .fold(src, |acc, x| acc.union(&x).cloned().collect())
  };
  let factor_first: Set = vec![
    Token::Identifier(Default::default()),
    Token::Integer(Default::default()),
    Token::ParL,
  ]
  .into_iter()
  .collect();
  let term_first = factor_first.clone();
  let exp_first: Set = term_first
    .clone()
    .union(&vec![Token::Add, Token::Sub].into_iter().collect())
    .cloned()
    .collect();
  let l_exp_first: Set = exp_first
    .clone()
    .union(&vec![Token::Odd].into_iter().collect())
    .cloned()
    .collect();
  let lop_first: Set = vec![
    Token::Eq,
    Token::Lt,
    Token::Gt,
    Token::Le,
    Token::Ge,
    Token::Ne,
  ]
  .into_iter()
  .collect();
  let aop_first: Set = vec![Token::Add, Token::Sub].into_iter().collect();
  let mop_first: Set = vec![Token::Mul, Token::Div].into_iter().collect();
  vec![
    (Field::Prog, prog_first),
    (Field::Block, block_first),
    (Field::ConstDecl, const_decl_first),
    (Field::Const, const_first),
    (Field::VarDecl, var_decl_first),
    (Field::Proc, proc_first),
    (Field::Body, body_first),
    (Field::Statement, statement_first),
    (Field::LExp, l_exp_first),
    (Field::Exp, exp_first),
    (Field::Term, term_first),
    (Field::Factor, factor_first),
    (Field::Lop, lop_first),
    (Field::Aop, aop_first),
    (Field::Mop, mop_first),
  ]
  .into_iter()
  .collect()
});

pub(crate) static TOKEN_FOLLOW_TABLE: Lazy<HashMap<Token, HashSet<Token>>> = Lazy::new(|| {
  type Set = HashSet<Token>;
  let id_token: Set = vec![Token::Identifier(Default::default())]
    .into_iter()
    .collect();
  let integer_token: Set = vec![Token::Integer(Default::default())]
    .into_iter()
    .collect();
  let end_follow = HashSet::default();
  let read_follow: Set = vec![Token::ParL].into_iter().collect();
  let write_follow = read_follow.clone();
  let aop_follow = FIELD_FIRST_TABLE.get(&Field::Term).unwrap().clone();
  let mop_follow = FIELD_FIRST_TABLE.get(&Field::Factor).unwrap().clone();
  let lop_follow = FIELD_FIRST_TABLE.get(&Field::Exp).unwrap().clone();
  let eqsign_follow = FIELD_FIRST_TABLE
    .get(&Field::Exp)
    .unwrap()
    .clone()
    .union(&integer_token)
    .cloned()
    .collect();
  let par_l_follow = FIELD_FIRST_TABLE
    .get(&Field::Exp)
    .unwrap()
    .clone()
    .union(&id_token)
    .cloned()
    .collect();
  let comma_follow = id_token
    .clone()
    .union(FIELD_FIRST_TABLE.get(&Field::Exp).unwrap())
    .cloned()
    .collect();
  let statement_follow: Set = vec![Token::Semicolon, Token::End, Token::Else]
    .into_iter()
    .collect();
  let factor_follow = FIELD_FIRST_TABLE.get(&Field::Mop).unwrap().clone();
  let par_r_follow = {
    let src: Set = vec![Token::Semicolon].into_iter().collect();
    let candidates = vec![factor_follow.clone(), statement_follow.clone()];
    candidates
      .into_iter()
      .fold(src, |acc, x| acc.union(&x).cloned().collect())
  };
  let semicolon_follow: Set = {
    let candidates = vec![
      FIELD_FIRST_TABLE.get(&Field::Block).unwrap().clone(),
      FIELD_FIRST_TABLE.get(&Field::Statement).unwrap().clone(),
      FIELD_FIRST_TABLE.get(&Field::Proc).unwrap().clone(),
    ];
    candidates
      .into_iter()
      .fold(Set::default(), |acc, x| acc.union(&x).cloned().collect())
  };
  let id_follow = vec![
    Token::Semicolon,
    Token::EqSign,
    Token::Comma,
    Token::ParL,
    Token::ParR,
  ]
  .into_iter()
  .collect();
  let integer_follow = Set::default();
  vec![
    (
      Token::If,
      FIELD_FIRST_TABLE.get(&Field::LExp).unwrap().clone(),
    ),
    (
      Token::Then,
      FIELD_FIRST_TABLE.get(&Field::Statement).unwrap().clone(),
    ),
    (
      Token::Else,
      FIELD_FIRST_TABLE.get(&Field::Statement).unwrap().clone(),
    ),
    (
      Token::While,
      FIELD_FIRST_TABLE.get(&Field::LExp).unwrap().clone(),
    ),
    (
      Token::Do,
      FIELD_FIRST_TABLE.get(&Field::Statement).unwrap().clone(),
    ),
    (Token::Const, id_token.clone()),
    (Token::Var, id_token.clone()),
    (Token::Procedure, id_token.clone()),
    (Token::Program, id_token.clone()),
    (
      Token::Begin,
      FIELD_FIRST_TABLE.get(&Field::Statement).unwrap().clone(),
    ),
    (Token::End, end_follow),
    (Token::Call, id_token.clone()),
    (Token::Read, read_follow),
    (Token::Write, write_follow),
    (
      Token::Odd,
      FIELD_FIRST_TABLE.get(&Field::Exp).unwrap().clone(),
    ),
    (Token::Add, aop_follow.clone()),
    (Token::Sub, aop_follow.clone()),
    (Token::Mul, mop_follow.clone()),
    (Token::Div, mop_follow.clone()),
    (Token::Eq, lop_follow.clone()),
    (Token::Lt, lop_follow.clone()),
    (Token::Gt, lop_follow.clone()),
    (Token::Le, lop_follow.clone()),
    (Token::Ge, lop_follow.clone()),
    (Token::Ne, lop_follow.clone()),
    (Token::EqSign, eqsign_follow),
    (Token::ParL, par_l_follow),
    (Token::ParR, par_r_follow),
    (Token::Semicolon, semicolon_follow),
    (Token::Comma, comma_follow),
    (Token::Identifier(Default::default()), id_follow),
    (Token::Integer(Default::default()), integer_follow),
  ]
  .into_iter()
  .collect()
});
