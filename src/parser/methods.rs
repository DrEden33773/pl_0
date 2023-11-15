use super::*;
use crate::{
  ast::*,
  parser::synchronizer::tables::{Field, FIELD_FOLLOW_TABLE},
};

impl<'a> Parser<'a> {
  /// ```bnf
  /// <prog> -> program <id> ; <block>
  pub(super) fn parse_program(&mut self) -> Option<Box<ProgramExpr>> {
    self.consume_next(Token::Program);
    let id = self.parse_id();
    self.consume_next(Token::Semicolon);
    let block = self.parse_block();
    if id.is_some() && block.is_some() {
      let (id, block) = (id.unwrap(), block.unwrap());
      Some(Box::new(ProgramExpr { id, block }))
    } else {
      None
    }
  }

  /// ```bnf
  /// <id> -> @letter { @letter | @digit }
  fn parse_id(&mut self) -> Option<Box<IdExpr>> {
    match self.consume_next_identifier() {
      Ok(id) => Some(Box::new(IdExpr(id))),
      Err(is_lexical_error) => {
        if is_lexical_error {
          while let Some(Token::LexicalError(_)) = self.lexer.peek() {
            self.consume_next_identifier().unwrap_err();
          }
        }
        let err = CompileErrorBuilder::syntax_error_template()
          .with_lexer_ref(&self.lexer)
          .with_info("Expected <id> field, but not found!".to_string())
          .build();
        eprintln!("{}", err);
        None
      }
    }
  }

  /// ```bnf
  /// <integer> -> @digit { @digit }
  fn parse_integer(&mut self) -> Option<Box<IntegerExpr>> {
    match self.consume_next_integer() {
      Ok(integer) => Some(Box::new(IntegerExpr(integer))),
      Err(is_lexical_error) => {
        if is_lexical_error {
          while let Some(Token::LexicalError(_)) = self.lexer.peek() {
            self.consume_next_integer().unwrap_err();
          }
        }
        let err = CompileErrorBuilder::syntax_error_template()
          .with_lexer_ref(&self.lexer)
          .with_info("Expected <integer> field, but not found!".to_string())
          .build();
        eprintln!("{}", err);
        None
      }
    }
  }

  /// ```bnf
  /// <block> -> [<const-decl>][<var-decl>][<proc>]<body>
  fn parse_block(&mut self) -> Option<Box<BlockExpr>> {
    // [<const-decl>]
    let const_decl = if self.match_next(Token::Const) {
      self.parse_const_decl()
    } else {
      None
    };
    // [<var-decl>]
    let var_decl = if self.match_next(Token::Var) {
      self.parse_var_decl()
    } else {
      None
    };
    // [<proc>]
    let proc = if self.match_next(Token::Procedure) {
      self.parse_proc()
    } else {
      None
    };
    let body = self.parse_body();
    body.map(|body| {
      Box::new(BlockExpr {
        const_decl,
        var_decl,
        proc,
        body,
      })
    })
  }

  /// ```bnf
  /// <const-decl> -> const <const> {, <const>} ;
  fn parse_const_decl(&mut self) -> Option<Box<ConstDeclExpr>> {
    let mut errored = false;
    let mut constants = vec![];
    self.consume_next(Token::Const);
    match self.parse_const() {
      Some(c) => constants.push(c),
      None => errored = true,
    }
    // {, <const>}
    while self.match_next(Token::Comma) {
      self.consume_next(Token::Comma);
      match self.parse_const() {
        Some(c) => constants.push(c),
        None => errored = true,
      }
    }
    if errored {
      return None;
    }
    self.consume_next(Token::Semicolon);
    Some(Box::new(ConstDeclExpr { constants }))
  }

  /// ```bnf
  /// <const> -> <id> := <integer>
  fn parse_const(&mut self) -> Option<Box<ConstExpr>> {
    let id_expr = self.parse_id();
    self.consume_next(Token::EqSign);
    let integer_expr = self.parse_integer();
    if id_expr.is_some() && integer_expr.is_some() {
      let (id, integer) = (id_expr.unwrap(), integer_expr.unwrap());
      Some(Box::new(ConstExpr { id, integer }))
    } else {
      None
    }
  }

  /// ```bnf
  /// <var-decl> -> var <id> {, <id>} ;
  fn parse_var_decl(&mut self) -> Option<Box<VarDeclExpr>> {
    let mut errored = false;
    let mut id_list = vec![];
    self.consume_next(Token::Var);
    match self.parse_id() {
      Some(id) => id_list.push(id),
      None => errored = true,
    }
    // {, <id>}
    while self.match_next(Token::Comma) {
      self.consume_next(Token::Comma);
      match self.parse_id() {
        Some(id) => id_list.push(id),
        None => errored = true,
      }
    }
    self.consume_next(Token::Semicolon);
    if errored {
      return None;
    }
    Some(Box::new(VarDeclExpr { id_list }))
  }

  /// ```bnf
  /// <proc> -> procedure <id> ([<id> {, <id>}]) ; <block> {; <proc>}
  fn parse_proc(&mut self) -> Option<Box<ProcExpr>> {
    self.consume_next(Token::Procedure);
    let id = self.parse_id();
    self.consume_next(Token::ParL);
    let mut args: Vec<Box<IdExpr>> = vec![];
    let mut errored = false;
    // [<id>]
    if !self.match_next(Token::ParR) {
      match self.parse_id() {
        Some(id) => args.push(id),
        None => errored = true,
      }
      // [<id> {, <id>}]
      while self.match_next(Token::Comma) {
        self.consume_next(Token::Comma);
        match self.parse_id() {
          Some(id) => args.push(id),
          None => errored = true,
        }
      }
      if errored {
        return None;
      }
    }
    self.consume_next(Token::ParR);
    self.consume_next(Token::Semicolon);
    let block = self.parse_block();
    let mut procs = vec![];
    // {; <proc>}
    while self.match_next(Token::Semicolon) {
      self.consume_next(Token::Semicolon);
      match self.parse_proc() {
        Some(proc) => procs.push(proc),
        None => errored = true,
      }
    }
    if errored {
      return None;
    }
    if id.is_some() && block.is_some() {
      let (id, block) = (id.unwrap(), block.unwrap());
      Some(Box::new(ProcExpr {
        id,
        args,
        block,
        procs,
      }))
    } else {
      None
    }
  }

  /// ```bnf
  /// <body> -> begin <statement> {; <statement>} end
  fn parse_body(&mut self) -> Option<Box<BodyExpr>> {
    let mut statements = vec![];
    let mut errored = false;
    self.consume_next(Token::Begin);
    match self.parse_statement() {
      Some(stmt) => statements.push(stmt),
      None => errored = true,
    }
    // {; <statement>}
    while self.match_next(Token::Semicolon) {
      self.consume_next(Token::Semicolon);
      match self.parse_statement() {
        Some(stmt) => statements.push(stmt),
        None => errored = true,
      }
    }
    if errored {
      return None;
    }
    self.consume_next(Token::End);
    Some(Box::new(BodyExpr { statements }))
  }

  /// ```bnf
  /// <statement> -> <id> := <exp>
  ///               | if <l-exp> then <statement> [else <statement>]
  ///               | while <l-exp> do <statement>
  ///               | call <id> ([<exp> {, <exp>}])
  ///               | read (<id> {, <id>})
  ///               | write (<exp> {, <exp>})
  ///               | <body>
  ///               | read (<id> {, <id>})
  ///               | write (<exp> {, <exp>})
  fn parse_statement(&mut self) -> Option<Box<StatementExpr>> {
    match self.lexer.peek() {
      Some(token) => match token {
        Token::If => {
          self.consume_next(Token::If);
          let l_exp = self.parse_l_exp();
          self.consume_next(Token::Then);
          let then_statement = self.parse_statement();
          let else_statement = if self.match_next(Token::Else) {
            self.consume_next(Token::Else);
            self.parse_statement()
          } else {
            None
          };
          if l_exp.is_some() && then_statement.is_some() {
            let (l_exp, then_statement) = (l_exp.unwrap(), then_statement.unwrap());
            Some(Box::new(StatementExpr::If {
              l_exp,
              then_statement,
              else_statement,
            }))
          } else {
            None
          }
        }
        Token::While => {
          self.consume_next(Token::While);
          let l_exp = self.parse_l_exp();
          self.consume_next(Token::Do);
          let statement = self.parse_statement();
          if l_exp.is_some() && statement.is_some() {
            let (l_exp, statement) = (l_exp.unwrap(), statement.unwrap());
            Some(Box::new(StatementExpr::While { l_exp, statement }))
          } else {
            None
          }
        }
        Token::Call => {
          let mut errored = false;
          self.consume_next(Token::Call);
          let id = self.parse_id();
          self.consume_next(Token::ParL);
          let mut args = vec![];
          // ([<exp>])
          if !self.match_next(Token::ParR) {
            match self.parse_exp() {
              Some(exp) => args.push(exp),
              None => errored = true,
            }
            // ([<exp> {, <exp>}])
            while self.match_next(Token::Comma) {
              self.consume_next(Token::Comma);
              match self.parse_exp() {
                Some(exp) => args.push(exp),
                None => errored = true,
              }
            }
          }
          self.consume_next(Token::ParR);
          if errored {
            return None;
          }
          if id.is_some() {
            let id = id.unwrap();
            Some(Box::new(StatementExpr::Call { id, args }))
          } else {
            None
          }
        }
        Token::Read => {
          let mut errored = false;
          self.consume_next(Token::Read);
          self.consume_next(Token::ParL);
          let mut id_list = vec![];
          match self.parse_id() {
            Some(id) => id_list.push(id),
            None => errored = true,
          }
          // (<id> {, <id>})
          while self.match_next(Token::Comma) {
            self.consume_next(Token::Comma);
            match self.parse_id() {
              Some(id) => id_list.push(id),
              None => errored = true,
            }
          }
          self.consume_next(Token::ParR);
          if errored {
            return None;
          }
          Some(Box::new(StatementExpr::Read { id_list }))
        }
        Token::Write => {
          let mut errored = false;
          self.consume_next(Token::Write);
          self.consume_next(Token::ParL);
          let mut exps = vec![];
          match self.parse_exp() {
            Some(exp) => exps.push(exp),
            None => errored = true,
          }
          // (<exp> {, <exp>})
          while self.match_next(Token::Comma) {
            self.consume_next(Token::Comma);
            match self.parse_exp() {
              Some(exp) => exps.push(exp),
              None => errored = true,
            }
          }
          self.consume_next(Token::ParR);
          if errored {
            return None;
          }
          Some(Box::new(StatementExpr::Write { exps }))
        }
        Token::Begin => {
          let body = self.parse_body();
          body.map(|body| Box::new(StatementExpr::Body { body }))
        }
        Token::Identifier(_) => {
          let id = self.parse_id();
          self.consume_next(Token::EqSign);
          let exp = self.parse_exp();
          if id.is_some() && exp.is_some() {
            let (id, exp) = (id.unwrap(), exp.unwrap());
            Some(Box::new(StatementExpr::Id { id, exp }))
          } else {
            None
          }
        }
        Token::LexicalError(_) => {
          self.has_error = true;
          while let Some(Token::LexicalError(_)) = self.lexer.peek() {
            self.consume_next_identifier().unwrap_err();
          }
          let err = CompileErrorBuilder::syntax_error_template()
            .with_lexer_ref(&self.lexer)
            .with_info("Expected <statement> field, but not found!".to_string())
            .build();
          self.consume_next(Token::EqSign);
          let _exp = self.parse_exp();
          eprintln!("{}", err);
          None
        }
        _ => {
          let unexpected_token = token.to_owned();
          let err = CompileErrorBuilder::syntax_error_template()
            .with_lexer_ref(&self.lexer)
            .with_info(format!(
              "Expected <statement> field, but got an unmatchable token `{}`",
              unexpected_token
            ))
            .build();
          eprintln!("{}", err);
          None
        }
      },
      None => {
        let err = CompileErrorBuilder::syntax_error_template()
          .with_lexer_ref(&self.lexer)
          .with_info("Expected <statement> field, but got `None`".to_string())
          .build();
        eprintln!("{}", err);
        None
      }
    }
  }

  /// ```bnf
  /// <l-exp> -> <exp> <lop> <exp> | odd <exp>
  fn parse_l_exp(&mut self) -> Option<Box<LExpExpr>> {
    if self.match_next(Token::Odd) {
      self.consume_next(Token::Odd);
      let exp = self.parse_exp();
      exp.map(|exp| Box::new(LExpExpr::Odd { exp }))
    } else {
      let l_exp = self.parse_exp();
      let lop = self.parse_lop();
      let r_exp = self.parse_exp();
      match (l_exp, lop, r_exp) {
        (Some(l_exp), Some(lop), Some(r_exp)) => {
          Some(Box::new(LExpExpr::Exp { l_exp, lop, r_exp }))
        }
        _ => None,
      }
    }
  }

  /// ```bnf
  /// <exp> -> [+|-] <term> {<aop> <term>}
  fn parse_exp(&mut self) -> Option<Box<ExpExpr>> {
    let is_next_add = self.match_next(Token::Add);
    let is_next_sub = self.match_next(Token::Sub);
    if is_next_add || is_next_sub {
      if is_next_add {
        self.consume_next(Token::Add);
      } else {
        self.consume_next(Token::Sub);
      }
    }
    let term = self.parse_term();
    let mut aop_terms = vec![];
    let mut errored = false;
    // {<aop> <term>}
    while self.match_next(Token::Add) || self.match_next(Token::Sub) {
      let aop = self.parse_aop();
      let term = self.parse_term();
      if aop.is_some() && term.is_some() {
        let (aop, term) = (aop.unwrap(), term.unwrap());
        aop_terms.push((aop, term));
      } else {
        errored = true;
      }
    }
    if errored {
      return None;
    }
    if term.is_some() {
      let term = term.unwrap();
      Some(Box::new(ExpExpr {
        is_negative: is_next_sub,
        term,
        aop_terms,
      }))
    } else {
      None
    }
  }

  /// ```bnf
  /// <term> -> <factor> {<mop> <factor>}
  fn parse_term(&mut self) -> Option<Box<TermExpr>> {
    let factor = self.parse_factor();
    let mut mop_factors = vec![];
    let mut errored = false;
    while self.match_next(Token::Mul) || self.match_next(Token::Div) {
      let mop = self.parse_mop();
      let factor = self.parse_factor();
      if mop.is_some() && factor.is_some() {
        let (mop, factor) = (mop.unwrap(), factor.unwrap());
        mop_factors.push((mop, factor));
      } else {
        errored = true;
      }
    }
    if errored {
      return None;
    }
    if factor.is_some() {
      let factor = factor.unwrap();
      Some(Box::new(TermExpr {
        factor,
        mop_factors,
      }))
    } else {
      None
    }
  }

  /// ```bnf
  /// <factor> -> <id> | <integer> | (<exp>)
  fn parse_factor(&mut self) -> Option<Box<FactorExpr>> {
    if self.match_next(Token::ParL) {
      self.consume_next(Token::ParL);
      let exp = self.parse_exp();
      self.consume_next(Token::ParR);
      exp.map(|exp| Box::new(FactorExpr::Exp(exp)))
    } else if matches!(self.lexer.peek(), Some(Token::Identifier(_))) {
      let id = self.parse_id();
      id.map(|id| Box::new(FactorExpr::Id(id)))
    } else if matches!(self.lexer.peek(), Some(Token::Integer(_))) {
      let integer = self.parse_integer();
      integer.map(|integer| Box::new(FactorExpr::Integer(integer)))
    } else if matches!(self.lexer.peek(), Some(Token::LexicalError(_))) {
      self.has_error = true;
      while let Some(Token::LexicalError(_)) = self.lexer.peek() {
        self.consume_next_identifier().unwrap_err();
      }
      let err = CompileErrorBuilder::syntax_error_template()
        .with_lexer_ref(&self.lexer)
        .with_info("Expected `<id>` / `<integer>` / `(<exp>)` field, but not found!".to_string())
        .build();
      eprintln!("{}", err);
      None
    } else {
      self.has_error = true;
      let unexpected_t = self.lexer.peek().cloned();
      let err = CompileErrorBuilder::syntax_error_template()
        .with_lexer_ref(&self.lexer)
        .with_info(format!(
          "Expected `<id>` / `<integer>` / `(<exp>)` field, but got an unmatchable token `{}`",
          match &unexpected_t {
            Some(t) => t.to_string(),
            None => "None".to_string(),
          }
        ))
        .build();
      eprintln!("{}", err);
      if let Some(t) = unexpected_t {
        if !FIELD_FOLLOW_TABLE.get(&Field::Factor).unwrap().contains(&t) {
          self.lexer.next();
        }
      } else {
        self.lexer.next();
      }
      None
    }
  }

  /// ```bnf
  /// <lop> -> = | <> | < | <= | > | >=
  fn parse_lop(&mut self) -> Option<Box<LopExpr>> {
    match self.lexer.peek() {
      Some(token) => match token {
        Token::Eq => {
          self.lexer.next();
          Some(Box::new(LopExpr::Eq))
        }
        Token::Lt => {
          self.lexer.next();
          Some(Box::new(LopExpr::Lt))
        }
        Token::Gt => {
          self.lexer.next();
          Some(Box::new(LopExpr::Gt))
        }
        Token::Le => {
          self.lexer.next();
          Some(Box::new(LopExpr::Le))
        }
        Token::Ge => {
          self.lexer.next();
          Some(Box::new(LopExpr::Ge))
        }
        Token::Ne => {
          self.lexer.next();
          Some(Box::new(LopExpr::Ne))
        }
        _ => {
          self.has_error = true;
          let unexpected_t = token.to_owned();
          let err = CompileErrorBuilder::syntax_error_template()
            .with_lexer_ref(&self.lexer)
            .with_info(format!(
              "Expected <lop> field, but got an unmatchable token `{}`",
              &unexpected_t
            ))
            .build();
          eprintln!("{}", err);
          if !FIELD_FOLLOW_TABLE
            .get(&Field::Lop)
            .unwrap()
            .contains(&unexpected_t)
          {
            self.lexer.next();
          }
          None
        }
      },
      None => {
        self.has_error = true;
        let err = CompileErrorBuilder::syntax_error_template()
          .with_lexer_ref(&self.lexer)
          .with_info("Expected <lop> field, but got `None`".to_string())
          .build();
        eprintln!("{}", err);
        self.lexer.next();

        None
      }
    }
  }

  /// ```bnf
  /// <aop> -> + | -
  fn parse_aop(&mut self) -> Option<Box<AopExpr>> {
    if self.match_next(Token::Add) {
      self.consume_next(Token::Add);
      Some(Box::new(AopExpr::Add))
    } else if self.match_next(Token::Sub) {
      self.consume_next(Token::Sub);
      Some(Box::new(AopExpr::Sub))
    } else {
      self.has_error = true;
      let unexpected_t = self.lexer.peek().cloned();
      let err = CompileErrorBuilder::syntax_error_template()
        .with_lexer_ref(&self.lexer)
        .with_info(format!(
          "Expected <aop> field, but got an unmatchable token `{}`",
          match &unexpected_t {
            Some(t) => t.to_string(),
            None => "None".to_string(),
          }
        ))
        .build();
      eprintln!("{}", err);
      if let Some(t) = unexpected_t {
        if !FIELD_FOLLOW_TABLE.get(&Field::Aop).unwrap().contains(&t) {
          self.lexer.next();
        }
      } else {
        self.lexer.next();
      }
      None
    }
  }

  /// ```bnf
  /// <mop> -> * | /
  fn parse_mop(&mut self) -> Option<Box<MopExpr>> {
    if self.match_next(Token::Mul) {
      self.consume_next(Token::Mul);
      Some(Box::new(MopExpr::Mul))
    } else if self.match_next(Token::Div) {
      self.consume_next(Token::Div);
      Some(Box::new(MopExpr::Div))
    } else {
      self.has_error = true;
      let unexpected_t = self.lexer.peek().cloned();
      let err = CompileErrorBuilder::syntax_error_template()
        .with_lexer_ref(&self.lexer)
        .with_info(format!(
          "Expected <mop> field, but got an unmatchable token `{}`",
          match &unexpected_t {
            Some(t) => t.to_string(),
            None => "None".to_string(),
          }
        ))
        .build();
      eprintln!("{}", err);
      if let Some(t) = unexpected_t {
        if !FIELD_FOLLOW_TABLE.get(&Field::Mop).unwrap().contains(&t) {
          self.lexer.next();
        }
      } else {
        self.lexer.next();
      }
      None
    }
  }
}
