#![allow(dead_code)]

use super::*;
use crate::ast::*;

impl<'a> Parser<'a> {
  /// ```bnf
  /// <prog> -> program <id> ; <block>
  pub(super) fn parse_program(&mut self) -> ProgramExpr {
    self.lexer.consume_next(Token::Program);
    let id = self.parse_id().into();
    self.lexer.consume_next(Token::Semicolon);
    let block = self.parse_block().into();
    ProgramExpr { id, block }
  }

  /// ```bnf
  /// <id> -> @letter { @letter | @digit }
  fn parse_id(&mut self) -> IdExpr {
    if let Some(Token::Identifier(id)) = self.lexer.next() {
      IdExpr(id)
    } else {
      self.lexer.panic_compile_error(
        CompileError::syntax_error_template(),
        "Expected <id> syntax_unit, but not found!".to_string(),
      )
    }
  }

  /// ```bnf
  /// <integer> -> @digit { @digit }
  fn parse_integer(&mut self) -> IntegerExpr {
    if let Some(Token::Integer(integer)) = self.lexer.next() {
      IntegerExpr(integer)
    } else {
      self.lexer.panic_compile_error(
        CompileError::syntax_error_template(),
        "Expected <integer> syntax_unit, but not found!".to_string(),
      )
    }
  }

  /// ```bnf
  /// <block> -> [<const-decl>][<var-decl>][<proc>]<body>
  fn parse_block(&mut self) -> BlockExpr {
    // [<const-decl>]
    let const_decl = if self.lexer.match_next(Token::Const) {
      Box::new(self.parse_const_decl()).into()
    } else {
      None
    };
    // [<var-decl>]
    let var_decl = if self.lexer.match_next(Token::Var) {
      Box::new(self.parse_var_decl()).into()
    } else {
      None
    };
    // [<proc>]
    let proc = if self.lexer.match_next(Token::Procedure) {
      Box::new(self.parse_proc()).into()
    } else {
      None
    };
    let body = self.parse_body().into();
    BlockExpr {
      const_decl,
      var_decl,
      proc,
      body,
    }
  }

  /// ```bnf
  /// <const-decl> -> const <const> {, <const>} ;
  fn parse_const_decl(&mut self) -> ConstDeclExpr {
    let mut constants = vec![];
    self.lexer.consume_next(Token::Const);
    constants.push(self.parse_const().into());
    // {, <const>}
    while self.lexer.match_next(Token::Comma) {
      self.lexer.consume_next(Token::Comma);
      constants.push(self.parse_const().into());
    }
    self.lexer.consume_next(Token::Semicolon);
    ConstDeclExpr { constants }
  }

  /// ```bnf
  /// <const> -> <id> := <integer>
  fn parse_const(&mut self) -> ConstExpr {
    let id_expr = self.parse_id();
    self.lexer.consume_next(Token::EqSign);
    let integer_expr = self.parse_integer();
    ConstExpr {
      id: id_expr.into(),
      integer: integer_expr.into(),
    }
  }

  /// ```bnf
  /// <var-decl> -> var <id> {, <id>} ;
  fn parse_var_decl(&mut self) -> VarDeclExpr {
    let mut ids = vec![];
    self.lexer.consume_next(Token::Var);
    ids.push(self.parse_id().into());
    // {, <id>}
    while self.lexer.match_next(Token::Comma) {
      self.lexer.consume_next(Token::Comma);
      ids.push(self.parse_id().into());
    }
    self.lexer.consume_next(Token::Semicolon);
    VarDeclExpr { ids }
  }

  /// ```bnf
  /// <proc> -> procedure <id> ([<id> {, <id>}]) ; <block> {; <proc>}
  fn parse_proc(&mut self) -> ProcExpr {
    self.lexer.consume_next(Token::Procedure);
    let id = self.parse_id().into();
    self.lexer.consume_next(Token::ParL);
    let mut args: Vec<Box<IdExpr>> = vec![];
    // [<id>]
    if !self.lexer.match_next(Token::ParR) {
      args.push(self.parse_id().into());
      // [<id> {, <id>}]
      while self.lexer.match_next(Token::Comma) {
        self.lexer.consume_next(Token::Comma);
        args.push(self.parse_id().into());
      }
    }
    self.lexer.consume_next(Token::ParR);
    self.lexer.consume_next(Token::Semicolon);
    let block = self.parse_block().into();
    let mut procs = vec![];
    // {; <proc>}
    while self.lexer.match_next(Token::Semicolon) {
      self.lexer.consume_next(Token::Semicolon);
      procs.push(self.parse_proc().into());
    }
    ProcExpr {
      id,
      args,
      block,
      procs,
    }
  }

  /// ```bnf
  /// <body> -> begin <statement> {; <statement>} end
  fn parse_body(&mut self) -> BodyExpr {
    let mut statements = vec![];
    self.lexer.consume_next(Token::Begin);
    statements.push(self.parse_statement().into());
    // {; <statement>}
    while self.lexer.match_next(Token::Semicolon) {
      self.lexer.consume_next(Token::Semicolon);
      statements.push(self.parse_statement().into());
    }
    self.lexer.consume_next(Token::End);
    BodyExpr { statements }
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
  fn parse_statement(&mut self) -> StatementExpr {
    match self.lexer.peek() {
      Some(token) => match token {
        Token::If => {
          self.lexer.consume_next(Token::If);
          let l_exp = self.parse_l_exp().into();
          self.lexer.consume_next(Token::Then);
          let then_statement = self.parse_statement().into();
          let else_statement = if self.lexer.match_next(Token::Else) {
            self.lexer.consume_next(Token::Else);
            Some(self.parse_statement().into())
          } else {
            None
          };
          StatementExpr::If {
            l_exp,
            then_statement,
            else_statement,
          }
        }
        Token::While => {
          self.lexer.consume_next(Token::While);
          let l_exp = self.parse_l_exp().into();
          self.lexer.consume_next(Token::Do);
          let statement = self.parse_statement().into();
          StatementExpr::While { l_exp, statement }
        }
        Token::Call => {
          self.lexer.consume_next(Token::Call);
          let id = self.parse_id().into();
          self.lexer.consume_next(Token::ParL);
          let mut args = vec![];
          // ([<exp>])
          if !self.lexer.match_next(Token::ParR) {
            args.push(self.parse_exp().into());
            // ([<exp> {, <exp>}])
            while self.lexer.match_next(Token::Comma) {
              self.lexer.consume_next(Token::Comma);
              args.push(self.parse_exp().into());
            }
          }
          self.lexer.consume_next(Token::ParR);
          StatementExpr::Call { id, args }
        }
        Token::Read => {
          self.lexer.consume_next(Token::Read);
          self.lexer.consume_next(Token::ParL);
          let mut ids = vec![];
          ids.push(self.parse_id().into());
          // (<id> {, <id>})
          while self.lexer.match_next(Token::Comma) {
            self.lexer.consume_next(Token::Comma);
            ids.push(self.parse_id().into());
          }
          self.lexer.consume_next(Token::ParR);
          StatementExpr::Read { ids }
        }
        Token::Write => {
          self.lexer.consume_next(Token::Write);
          self.lexer.consume_next(Token::ParL);
          let mut exps = vec![];
          exps.push(self.parse_exp().into());
          // (<exp> {, <exp>})
          while self.lexer.match_next(Token::Comma) {
            self.lexer.consume_next(Token::Comma);
            exps.push(self.parse_exp().into());
          }
          self.lexer.consume_next(Token::ParR);
          StatementExpr::Write { exps }
        }
        Token::Begin => {
          let body = self.parse_body().into();
          StatementExpr::Body { body }
        }
        Token::Identifier(_) => {
          let id = self.parse_id().into();
          self.lexer.consume_next(Token::EqSign);
          let exp = self.parse_exp().into();
          StatementExpr::Id { id, exp }
        }
        _ => {
          let unexpected_token = token.to_owned();
          self.lexer.panic_compile_error(
            CompileError::syntax_error_template(),
            format!(
              "Expected <statement> syntax_unit, but got an unmatchable token `{:?}`",
              unexpected_token
            ),
          );
        }
      },
      None => self.lexer.panic_compile_error(
        CompileError::syntax_error_template(),
        "Expected <statement> syntax_unit, but got `None`".to_string(),
      ),
    }
  }

  /// ```bnf
  /// <l-exp> -> <exp> <lop> <exp> | odd <exp>
  fn parse_l_exp(&mut self) -> LExpExpr {
    if self.lexer.match_next(Token::Odd) {
      self.lexer.consume_next(Token::Odd);
      let exp = self.parse_exp().into();
      LExpExpr::Odd { exp }
    } else {
      let l_exp = self.parse_exp().into();
      let lop = self.parse_lop().into();
      let r_exp = self.parse_exp().into();
      LExpExpr::Exp { l_exp, lop, r_exp }
    }
  }

  /// ```bnf
  /// <exp> -> [+|-] <term> {<aop> <term>}
  fn parse_exp(&mut self) -> ExpExpr {
    let is_next_add = self.lexer.match_next(Token::Add);
    let is_next_sub = self.lexer.match_next(Token::Sub);
    if is_next_add || is_next_sub {
      if is_next_add {
        self.lexer.consume_next(Token::Add);
      } else {
        self.lexer.consume_next(Token::Sub);
      }
    }
    let term = self.parse_term().into();
    let mut aop_terms = vec![];
    // {<aop> <term>}
    while self.lexer.match_next(Token::Add) || self.lexer.match_next(Token::Sub) {
      let aop = self.parse_aop();
      let term = self.parse_term();
      aop_terms.push((aop.into(), term.into()));
    }
    ExpExpr {
      is_negative: is_next_sub,
      term,
      aop_terms,
    }
  }

  /// ```bnf
  /// <term> -> <factor> {<mop> <factor>}
  fn parse_term(&mut self) -> TermExpr {
    let factor = self.parse_factor().into();
    let mut mop_factors = vec![];
    while self.lexer.match_next(Token::Mul) || self.lexer.match_next(Token::Div) {
      let mop = self.parse_mop();
      let factor = self.parse_factor();
      mop_factors.push((mop.into(), factor.into()));
    }
    TermExpr {
      factor,
      mop_factors,
    }
  }

  /// ```bnf
  /// <factor> -> <id> | <integer> | (<exp>)
  fn parse_factor(&mut self) -> FactorExpr {
    if self.lexer.match_next(Token::ParL) {
      self.lexer.consume_next(Token::ParL);
      let exp = self.parse_exp().into();
      self.lexer.consume_next(Token::ParR);
      FactorExpr::Exp(exp)
    } else if matches!(self.lexer.peek(), Some(Token::Identifier(_))) {
      let id = self.parse_id().into();
      FactorExpr::Id(id)
    } else if matches!(self.lexer.peek(), Some(Token::Integer(_))) {
      let integer = self.parse_integer().into();
      FactorExpr::Integer(integer)
    } else {
      // BUG: `self.lexer.next()` / `self.lexer.peek().cloned()`, which one?
      let unexpected_token = self.lexer.next();
      self.lexer.panic_compile_error(
        CompileError::syntax_error_template(),
        format!(
          "Expected <id> / <integer> / (<exp>) syntax_unit, but got an unmatchable token `{:?}`",
          unexpected_token
        ),
      )
    }
  }

  /// ```bnf
  /// <lop> -> = | <> | < | <= | > | >=
  fn parse_lop(&mut self) -> LopExpr {
    match self.lexer.next() {
      Some(token) => match token {
        Token::Eq => LopExpr::Eq,
        Token::Lt => LopExpr::Lt,
        Token::Gt => LopExpr::Gt,
        Token::Le => LopExpr::Le,
        Token::Ge => LopExpr::Ge,
        Token::Ne => LopExpr::Ne,
        _ => {
          let unexpected_token = token.to_owned();
          self.lexer.panic_compile_error(
            CompileError::syntax_error_template(),
            format!(
              "Expected <lop> syntax_unit, but got an unmatchable token `{:?}`",
              unexpected_token
            ),
          );
        }
      },
      None => {
        self.lexer.panic_compile_error(
          CompileError::syntax_error_template(),
          "Expected <statement> syntax_unit, but got `None`".to_string(),
        );
      }
    }
  }

  /// ```bnf
  /// <aop> -> + | -
  fn parse_aop(&mut self) -> AopExpr {
    if self.lexer.match_next(Token::Add) {
      self.lexer.consume_next(Token::Add);
      AopExpr::Add
    } else if self.lexer.match_next(Token::Sub) {
      self.lexer.consume_next(Token::Sub);
      AopExpr::Sub
    } else {
      let unexpected_token = self.lexer.next();
      self.lexer.panic_compile_error(
        CompileError::syntax_error_template(),
        format!(
          "Expected <aop> syntax_unit, but got an unmatchable token `{:?}`",
          unexpected_token
        ),
      );
    }
  }

  /// ```bnf
  /// <mop> -> * | /
  fn parse_mop(&mut self) -> MopExpr {
    if self.lexer.match_next(Token::Mul) {
      self.lexer.consume_next(Token::Mul);
      MopExpr::Mul
    } else if self.lexer.match_next(Token::Div) {
      self.lexer.consume_next(Token::Div);
      MopExpr::Div
    } else {
      let unexpected_token = self.lexer.next();
      self.lexer.panic_compile_error(
        CompileError::syntax_error_template(),
        format!(
          "Expected <mop> syntax_unit, but got an unmatchable token `{:?}`",
          unexpected_token
        ),
      );
    }
  }
}
