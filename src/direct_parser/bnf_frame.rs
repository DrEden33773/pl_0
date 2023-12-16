use super::{desc::ExprDesc, *};

#[allow(unused)]
impl<'a> DirectParser<'a> {
  /// ```bnf
  /// <prog> -> program <id> ; <block>
  fn program(&mut self) {
    self.consume_next(Token::Program);
    let id = self.id();
    self.consume_next(Token::Semicolon);
  }

  /// ```bnf
  /// <block> -> [<const-decl>][<var-decl>][<proc>]<body>
  fn block(&mut self) {}

  /// ```bnf
  /// <const-decl> -> const <const> {, <const>} ;
  fn const_decl(&mut self) {}

  /// ```bnf
  /// <var-decl> -> var <id> {, <id>} ;
  fn var_decl(&mut self) {}

  /// ```bnf
  /// <proc> -> procedure <id> ([<id> {, <id>}]) ; <block> {; <proc>}
  fn proc(&mut self) {}

  /// ```bnf
  /// <body> -> begin <statement> {; <statement>} end
  fn body(&mut self) {}

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
  fn statement(&mut self) {}

  /// ```bnf
  /// <l-exp> -> <exp> <lop> <exp> | odd <exp>
  pub(super) fn l_exp(&mut self) -> ExprDesc {
    unimplemented!()
  }

  /// ```bnf
  /// <exp> -> [+|-] <term> {<aop> <term>}
  fn exp(&mut self) -> ExprDesc {
    unimplemented!()
  }

  /// ```bnf
  /// <term> -> <factor> {<mop> <factor>}
  fn term(&mut self) -> ExprDesc {
    unimplemented!()
  }

  /// ```bnf
  /// <factor> -> <id> | <integer> | (<exp>)
  fn factor(&mut self) -> ExprDesc {
    unimplemented!()
  }

  /// ```bnf
  /// <lop> -> = | <> | < | <= | > | >=
  fn lop(&mut self) -> Option<Token> {
    unimplemented!()
  }

  /// ```bnf
  /// <aop> -> + | -
  fn aop(&mut self) -> Option<Token> {
    unimplemented!()
  }

  /// ```bnf
  /// <mop> -> * | /
  fn mop(&mut self) -> Option<Token> {
    unimplemented!()
  }

  /// ```bnf
  /// <id> -> @letter { @letter | @digit }
  fn id(&mut self) -> Option<String> {
    match self.consume_next_identifier() {
      Ok(id) => id.into(),
      Err(is_lexical_err) => {
        if is_lexical_err {
          while let Some(Token::LexicalError(_)) = self.ctx.lexer.peek() {
            self.consume_next_identifier().unwrap_err();
          }
        }
        let err = CompileErrorBuilder::syntax_error_template()
          .with_lexer_ref(&self.ctx.lexer)
          .with_info("Expected <id> field, but not found!".to_string())
          .build();
        eprintln!("{}", err);
        None
      }
    }
  }

  /// ```bnf
  /// <integer> -> @digit { @digit }
  fn integer(&mut self) -> Option<i64> {
    match self.consume_next_integer() {
      Ok(integer) => integer.into(),
      Err(is_lexical_err) => {
        if is_lexical_err {
          while let Some(Token::LexicalError(_)) = self.ctx.lexer.peek() {
            self.consume_next_integer().unwrap_err();
          }
        }
        let err = CompileErrorBuilder::syntax_error_template()
          .with_lexer_ref(&self.ctx.lexer)
          .with_info("Expected <integer> field, but not found!".to_string())
          .build();
        eprintln!("{}", err);
        None
      }
    }
  }
}
