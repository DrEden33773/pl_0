use super::*;

impl<'a> Parser<'a> {
  /// ```bnf
  /// <prog> -> program <id> ; <block>
  pub(super) fn parse_program(&mut self) -> ParseResult {
    self.lexer.consume_next(Token::Program);
    self.parse_id();
    self.lexer.consume_next(Token::Semicolon);
    self.parse_block();
  }

  /// ```bnf
  /// <id> -> @letter { @letter | @digit }
  fn parse_id(&mut self) -> ParseResult {
    if matches!(self.lexer.next(), Some(Token::Identifier(_))) {
    } else {
      self.lexer.panic_compile_error(
        CompileError::syntax_error_template(),
        "Expected <id> token, but not found!".to_string(),
      )
    }
  }

  /// ```bnf
  /// <integer> -> @digit { @digit }
  fn parse_integer(&mut self) -> ParseResult {
    if matches!(self.lexer.next(), Some(Token::Integer(_))) {
    } else {
      self.lexer.panic_compile_error(
        CompileError::syntax_error_template(),
        "Expected <integer> token, but not found!".to_string(),
      )
    }
  }

  /// ```bnf
  /// <block> -> [<const-decl>][<var-decl>][<proc>]<body>
  fn parse_block(&mut self) -> ParseResult {
    // [<const-decl>]
    if self.lexer.match_next(Token::Const) {
      self.parse_const_decl();
    }
    // [<var-decl>]
    if self.lexer.match_next(Token::Var) {
      self.parse_var_decl();
    }
    // [<proc>]
    if self.lexer.match_next(Token::Procedure) {
      self.parse_proc();
    }
    self.parse_body();
  }

  /// ```bnf
  /// <const-decl> -> const <const> {, <const>} ;
  fn parse_const_decl(&mut self) -> ParseResult {
    self.lexer.consume_next(Token::Const);
    self.parse_const();
    // {, <const>}
    while self.lexer.match_next(Token::Comma) {
      self.lexer.consume_next(Token::Comma);
      self.parse_const();
    }
    self.lexer.consume_next(Token::Semicolon);
  }

  /// ```bnf
  /// <const> -> <id> := <integer>
  fn parse_const(&mut self) -> ParseResult {
    self.parse_id();
    self.lexer.consume_next(Token::EqSign);
    self.parse_integer();
  }

  /// ```bnf
  /// <var-decl> -> var <id> {, <id>} ;
  fn parse_var_decl(&mut self) -> ParseResult {
    self.lexer.consume_next(Token::Var);
    self.parse_id();
    // {, <id>}
    while self.lexer.match_next(Token::Comma) {
      self.lexer.consume_next(Token::Comma);
      self.parse_id();
    }
    self.lexer.consume_next(Token::Semicolon);
  }

  /// ```bnf
  /// <proc> -> procedure <id> ( [<id> {, <id>}] ) ; <block> {; <proc>}
  fn parse_proc(&mut self) -> ParseResult {
    self.lexer.consume_next(Token::Procedure);
    self.parse_id();
    self.lexer.consume_next(Token::ParL);
    // [<id>]
    if !self.lexer.match_next(Token::ParR) {
      self.parse_id();
      // [<id> {, <id>}]
      while self.lexer.match_next(Token::Comma) {
        self.lexer.consume_next(Token::Comma);
        self.parse_id();
      }
    }
    self.lexer.consume_next(Token::ParR);
    self.lexer.consume_next(Token::Semicolon);
    self.parse_block();
    // {; <proc>}
    while self.lexer.match_next(Token::Semicolon) {
      self.lexer.consume_next(Token::Semicolon);
      self.parse_proc();
    }
  }

  /// ```bnf
  /// <body> -> begin <statement> {; <statement>} end
  fn parse_body(&mut self) -> ParseResult {
    self.lexer.consume_next(Token::Begin);
    self.parse_statement();
    // {; <statement>}
    while self.lexer.match_next(Token::Semicolon) {
      self.lexer.consume_next(Token::Semicolon);
      self.parse_statement();
    }
    self.lexer.consume_next(Token::End);
  }

  /// ```bnf
  /// <statement> -> <id> := <exp>
  ///               | if <l-exp> then <statement> [else <statement>]
  ///               | while <l-exp> do <statement>
  ///               | call <id> ( [<exp> {, <exp>}] )
  ///               | read ( <id> {, <id>} )
  ///               | write ( <exp> {, <exp>} )
  ///               | <body>
  ///               | read ( <id> {, <id>} )
  ///               | write ( <exp> {, <exp>} )
  fn parse_statement(&mut self) -> ParseResult {
    match self.lexer.peek() {
      Some(token) => match token {
        Token::If => {
          self.lexer.consume_next(Token::If);
          self.parse_l_exp();
          self.lexer.consume_next(Token::Then);
          self.parse_statement();
          if self.lexer.match_next(Token::Else) {
            self.lexer.consume_next(Token::Else);
            self.parse_statement();
          }
        }
        Token::While => {
          self.lexer.consume_next(Token::While);
          self.parse_l_exp();
          self.lexer.consume_next(Token::Do);
          self.parse_statement();
        }
        Token::Call => {
          self.lexer.consume_next(Token::Call);
          self.parse_id();
          self.lexer.consume_next(Token::ParL);
          // ( [<exp>] )
          if !self.lexer.match_next(Token::ParR) {
            self.parse_exp();
            // ( [<exp> {, <exp>}] )
            while self.lexer.match_next(Token::Comma) {
              self.lexer.consume_next(Token::Comma);
              self.parse_exp();
            }
          }
          self.lexer.consume_next(Token::ParR);
        }
        Token::Read => {
          self.lexer.consume_next(Token::Read);
          self.lexer.consume_next(Token::ParL);
          self.parse_id();
          // ( <id> {, <id>} )
          while self.lexer.match_next(Token::Comma) {
            self.lexer.consume_next(Token::Comma);
            self.parse_id();
          }
          self.lexer.consume_next(Token::ParR);
        }
        Token::Write => {
          self.lexer.consume_next(Token::Write);
          self.lexer.consume_next(Token::ParL);
          self.parse_exp();
          // ( <exp> {, <exp>} )
          while self.lexer.match_next(Token::Comma) {
            self.lexer.consume_next(Token::Comma);
            self.parse_exp();
          }
          self.lexer.consume_next(Token::ParR);
        }
        Token::Begin => self.parse_body(),
        Token::Identifier(_) => {
          self.parse_id();
          self.lexer.consume_next(Token::EqSign);
          self.parse_exp();
        }
        _ => {
          let unexpected_token = token.to_owned();
          self.lexer.panic_compile_error(
            CompileError::syntax_error_template(),
            format!(
              "Expected a <statement> syntax_unit, but got an illegal token `{:?}`",
              unexpected_token
            ),
          );
        }
      },
      None => self.lexer.panic_compile_error(
        CompileError::syntax_error_template(),
        "Expected a <statement> syntax_unit, but got `None`".to_string(),
      ),
    }
  }

  /// ```bnf
  /// <l-exp> -> <exp> <lop> <exp> | odd <exp>
  fn parse_l_exp(&mut self) -> ParseResult {
    if self.lexer.match_next(Token::Odd) {
      self.lexer.consume_next(Token::Odd);
      self.parse_exp();
    } else {
      self.parse_exp();
      self.parse_lop();
      self.parse_exp();
    }
  }

  /// ```bnf
  /// <exp> -> [+|-] <term> {<aop> <term>}
  fn parse_exp(&mut self) -> ParseResult {
    let is_next_add = self.lexer.match_next(Token::Add);
    let is_next_sub = self.lexer.match_next(Token::Sub);
    if is_next_add || is_next_sub {
      if is_next_add {
        self.lexer.consume_next(Token::Add);
      } else {
        self.lexer.consume_next(Token::Sub);
      }
    }
    self.parse_term();
    // {<aop> <term>}
    while self.lexer.match_next(Token::Add) || self.lexer.match_next(Token::Sub) {
      self.parse_aop();
      self.parse_term();
    }
  }

  /// ```bnf
  /// <term> -> <factor> {<mop> <factor>}
  fn parse_term(&mut self) -> ParseResult {
    self.parse_factor();
    while self.lexer.match_next(Token::Mul) || self.lexer.match_next(Token::Div) {
      self.parse_mop();
      self.parse_factor();
    }
  }

  /// ```bnf
  /// <factor> -> <id> | <integer> | (<exp>)
  fn parse_factor(&mut self) -> ParseResult {
    if self.lexer.match_next(Token::ParL) {
      self.lexer.consume_next(Token::ParL);
      self.parse_exp();
      self.lexer.consume_next(Token::ParR);
    } else if matches!(self.lexer.peek(), Some(Token::Identifier(_))) {
      self.parse_id();
    } else if matches!(self.lexer.peek(), Some(Token::Integer(_))) {
      self.parse_integer();
    }
  }

  /// ```bnf
  /// <lop> -> = | <> | < | <= | > | >=
  fn parse_lop(&mut self) -> ParseResult {
    match self.lexer.next() {
      Some(token) => match token {
        Token::Eq => { /* TODO */ }
        Token::Lt => { /* TODO */ }
        Token::Gt => { /* TODO */ }
        Token::Le => { /* TODO */ }
        Token::Ge => { /* TODO */ }
        Token::Ne => { /* TODO */ }
        _ => {
          let unexpected_token = token.to_owned();
          self.lexer.panic_compile_error(
            CompileError::syntax_error_template(),
            format!(
              "Expected a <lop> syntax_unit, but got an illegal token `{:?}`",
              unexpected_token
            ),
          );
        }
      },
      None => {
        self.lexer.panic_compile_error(
          CompileError::syntax_error_template(),
          "Expected a <statement> syntax_unit, but got an illegal token `None`".to_string(),
        );
      }
    }
  }

  /// ```bnf
  /// <aop> -> + | -
  fn parse_aop(&mut self) -> ParseResult {
    if self.lexer.match_next(Token::Add) {
      self.lexer.consume_next(Token::Add);
    } else if self.lexer.match_next(Token::Sub) {
      self.lexer.consume_next(Token::Sub);
    } else {
      self.lexer.panic_compile_error(
        CompileError::syntax_error_template(),
        "Expected an <aop> syntax_unit, but got `None`".to_string(),
      );
    }
  }

  /// ```bnf
  /// <mop> -> * | /
  fn parse_mop(&mut self) -> ParseResult {
    if self.lexer.match_next(Token::Mul) {
      self.lexer.consume_next(Token::Mul);
    } else if self.lexer.match_next(Token::Div) {
      self.lexer.consume_next(Token::Div);
    } else {
      self.lexer.panic_compile_error(
        CompileError::syntax_error_template(),
        "Expected an <mop> syntax_unit, but got `None`".to_string(),
      );
    }
  }
}
