#![allow(unused)]

pub mod desc;

use std::assert_matches::{assert_matches, debug_assert_matches};

use self::desc::{ActivationRecord, ExprDesc, Level};
use crate::{
  ast::{
    BlockExpr, ConstDeclExpr, ExpExpr, LExpExpr, ProcExpr, ProgramExpr, StatementExpr, VarDeclExpr,
  },
  bytecode::advanced::ByteCode,
  lexer::Lexer,
};

struct CodegenContext<'a> {
  /// Marking original code (for better error handling)
  lexer: Lexer<'a>,
  /// Levels of current context
  levels: Vec<Level>,
}

impl<'a> CodegenContext<'a> {
  fn new(ctx: &'a str) -> Self {
    Self {
      lexer: Lexer::new(ctx),
      levels: vec![],
    }
  }
}

/// - l_ctx: ctx's *lifetime*
/// - l_lex: ctx.lexer's *lifetime*
///
/// l_lex `is longer than` l_ctx
pub struct TreeWalkCodeGenerator<'l_ctx, 'l_lex: 'l_ctx> {
  /// Marking original code (for better error handling)
  ctx: &'l_ctx mut CodegenContext<'l_lex>,
  /// The only legal entrance ast_node
  ast_entry: Box<ProgramExpr>,
  /// Aka. stack pointer
  sp: usize,
  /// Aka. activation_record
  ar: ActivationRecord,

  break_blocks: Vec<Vec<usize>>,
  continue_blocks: Vec<Vec<(usize, usize)>>,
}

impl<'a, 'l: 'a> TreeWalkCodeGenerator<'a, 'l> {
  fn program(&mut self) {
    let n_var = self.local_num();
    self.program_scope();
    self.local_expire(n_var);
  }

  fn program_scope(&mut self) {}

  fn block(&mut self, expr: &BlockExpr) {}

  fn const_decl(&mut self, expr: &ConstDeclExpr) {}

  fn var_decl(&mut self, expr: &VarDeclExpr) {}

  fn proc(&mut self, expr: &ProcExpr) {}
}

impl<'a, 'l: 'a> TreeWalkCodeGenerator<'a, 'l> {
  fn l_exp(&mut self, expr: &LExpExpr) -> ExprDesc {
    unimplemented!()
  }

  fn exp(&mut self, expr: &ExpExpr) -> ExprDesc {
    unimplemented!()
  }
}

impl<'a, 'l: 'a> TreeWalkCodeGenerator<'a, 'l> {
  /// ```bnf
  /// <statement>
  ///   -> <id> := <exp>
  ///   | if <l-exp> then <statement> [else <statement>]
  ///   | while <l-exp> do <statement>
  ///   | call <id> ([<exp> {, <exp>}])
  ///   | <body>
  ///   | read (<id> {, <id>})
  ///   | write (<exp> {, <exp>})
  fn stat(&mut self, expr: &StatementExpr) {}

  /// ```bnf
  /// while <l-exp> do <statement>
  fn while_stat(&mut self, expr: &StatementExpr) {
    if let StatementExpr::While { l_exp, statement } = expr.to_owned() {
      let i_start = self.ar.byte_codes.len();

      let condition = self.l_exp(&l_exp);
      let false_list = self.test_or_jump(condition);

      self.stat(&statement);

      // jump back
      let i_end = self.ar.byte_codes.len();

      self
        .ar
        .byte_codes
        .push(ByteCode::Jump(-((i_end - i_start) as i16) - 1));

      self.fix_test_list(false_list);
    }
  }

  /// ```bnf
  /// if <l-exp> then <statement> [else <statement>]
  fn if_stat(&mut self, expr: &StatementExpr) {
    if let StatementExpr::If {
      l_exp,
      then_statement,
      else_statement,
    } = expr.to_owned()
    {
      let mut jmp_ends = vec![];
      self.do_if_block(&mut jmp_ends, expr);

      if let Some(expr) = else_statement {
        self.stat(&expr)
      }

      let i_end = self.ar.byte_codes.len() - 1;
      for i in jmp_ends {
        self.ar.byte_codes[i] = ByteCode::Jump((i_end - 1) as i16);
      }
    }
  }

  fn do_if_block(&mut self, jmp_ends: &mut Vec<usize>, expr: &StatementExpr) {
    if let StatementExpr::If {
      l_exp,
      then_statement,
      else_statement,
    } = expr.to_owned()
    {
      let condition = self.l_exp(&l_exp);
      let false_list = self.test_or_jump(condition);

      if else_statement.is_some() {
        self.ar.byte_codes.push(ByteCode::Jump(0));
        jmp_ends.push(self.ar.byte_codes.len() - 1);
      }

      self.fix_test_list(false_list);
    }
  }
}

impl<'a, 'l: 'a> TreeWalkCodeGenerator<'a, 'l> {
  fn test_or_jump(&mut self, condition: ExprDesc) -> Vec<usize> {
    unimplemented!()
  }

  // fix TestAndJump/TestOrJump list to jump to current place
  fn fix_test_list(&mut self, list: Vec<usize>) {
    let here = self.ar.byte_codes.len();
    self.fix_test_list_to(list, here);
  }

  // fix TestAndJump/TestOrJump list to jump to $to
  fn fix_test_list_to(&mut self, list: Vec<usize>, to: usize) {
    for i in list {
      let jmp = (to as isize - i as isize - 1) as i16;
      let code = match self.ar.byte_codes[i] {
        ByteCode::Jump(0) => ByteCode::Jump(jmp),
        ByteCode::TestOrJump(i_condition, 0) => ByteCode::TestOrJump(i_condition, jmp),
        ByteCode::TestAndJump(i_condition, 0) => ByteCode::TestAndJump(i_condition, jmp),
        _ => panic!("invalid test"),
      };
      self.ar.byte_codes[i] = code;
    }
  }
}

impl<'a, 'l: 'a> TreeWalkCodeGenerator<'a, 'l> {
  fn local_expire(&mut self, from: usize) {
    // drop locals
    let mut vars = self.ctx.levels.last_mut().unwrap().locals.drain(from..);
    // generate Close if any dropped local variable referred as upvalue
    if vars.any(|(name, is_up_value)| is_up_value) {
      self.ar.byte_codes.push(ByteCode::Close(from as u8));
    }
  }

  fn local_new(&mut self, name: String) {
    self
      .ctx
      .levels
      .last_mut()
      .unwrap()
      .locals
      .push((name, false));
  }

  fn local_num(&self) -> usize {
    self.ctx.levels.last().unwrap().locals.len()
  }
}

fn chunk<'a, 'l: 'a>(
  ctx: &'a mut CodegenContext<'l>,
  has_var_args: bool,
  params: Vec<String>,
  ast_entry: Box<ProgramExpr>,
) -> ActivationRecord {
  // prepare
  let ar = ActivationRecord {
    has_var_args,
    n_param: params.len(),
    ..Default::default()
  };
  ctx.levels.push(Level {
    locals: params.into_iter().map(|p| (p, false)).collect(),
    up_values: vec![],
  });
  let generator = TreeWalkCodeGenerator {
    ctx,
    ast_entry,
    sp: 0,
    ar,
    break_blocks: vec![],
    continue_blocks: vec![],
  };

  // generate

  // clear
  let TreeWalkCodeGenerator { mut ar, ctx, .. } = generator;
  let level = ctx.levels.pop().unwrap();
  ar.up_indexes = level.up_values.into_iter().map(|(_, i)| i).collect();

  ar
}

pub fn load(lexer_ref: &Lexer, ast_entry: Box<ProgramExpr>) -> ActivationRecord {
  let mut ctx = CodegenContext {
    lexer: lexer_ref.to_owned(),
    levels: vec![],
  };
  chunk(&mut ctx, false, vec!["_ENV".into()], ast_entry)
}
