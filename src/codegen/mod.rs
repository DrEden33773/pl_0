#![allow(unused)]

pub mod desc;

use std::{ops::Deref, rc::Rc};

use self::desc::{ActivationRecord, ConstStack, ExprDesc, Level};
use crate::{
  ast::{
    AstExpr, BlockExpr, ConstDeclExpr, ExpExpr, LExpExpr, ProcExpr, ProgramExpr, StatementExpr,
    VarDeclExpr,
  },
  bytecode::advanced::ByteCode,
  codegen::desc::UpIndex,
  error::compile_error::CompileError,
  lexer::Lexer,
  value::Value,
};

struct CodegenContext {
  /// Levels of current context
  levels: Vec<Level>,
}

impl CodegenContext {
  fn new() -> Self {
    Self { levels: vec![] }
  }
}

/// - l_ctx: ctx's *lifetime*
/// - l_lex: ctx.lexer's *lifetime*
///
/// l_lex `is longer than` l_ctx
pub struct TreeWalkCodeGenerator<'l_ctx> {
  /// Marking original code (for better error handling)
  ctx: &'l_ctx mut CodegenContext,
  /// The only legal entrance ast_node
  curr_ast: AstExpr,
  /// Aka. stack pointer
  sp: usize,
  /// Aka. activation_record
  ar: ActivationRecord,
}

impl<'a> TreeWalkCodeGenerator<'a> {
  fn program(&mut self) {
    let n_var = self.local_num();
    self.program_scope();
    self.local_expire(n_var);
  }

  fn program_scope(&mut self) {}

  fn block(&mut self, expr: &BlockExpr) {}

  fn const_decl(&mut self, expr: &ConstDeclExpr) {}

  fn var_decl(&mut self, expr: &VarDeclExpr) {}

  fn proc(&mut self, expr: &ProcExpr) {
    self.func_def(expr)
  }
}

impl<'a> TreeWalkCodeGenerator<'a> {
  /// ```bnf
  /// <proc> -> procedure <id> ([<id> {, <id>}]) ; <block> {; <proc>}
  fn func_def(&mut self, expr: &ProcExpr) {
    let name = expr.id.as_ref().0.to_owned();
    let name_info = expr.id.as_ref().1;
    let mut name_desc = self.simple_name(name);

    let body = self.func_body(expr);
  }

  // () / (id) / (id, id, ...)
  fn func_body(&mut self, expr: &ProcExpr) -> ExprDesc {
    // parameter list
    let mut has_var_args = false;
    let mut params = expr
      .args
      .iter()
      .map(|id| id.0.to_owned())
      .collect::<Vec<_>>();

    // body(block)
    // BUG
    let ar = chunk(self.ctx, has_var_args, params, &expr.block);

    let has_upvalue = !ar.up_indexes.is_empty();
    let i_const = self.add_const(Value::PL0Function(Rc::new(ar)));
    if has_upvalue {
      ExprDesc::Closure(i_const)
    } else {
      ExprDesc::Function(i_const)
    }
  }
}

impl<'a> TreeWalkCodeGenerator<'a> {
  fn l_exp(&mut self, expr: &LExpExpr) -> ExprDesc {
    fn is_odd(i: i64) -> bool {
      i % 2 == 1
    };
    match expr {
      LExpExpr::Exp { l_exp, lop, r_exp } => {
        todo!()
      }
      LExpExpr::Odd { exp } => {
        todo!()
      }
    }
  }

  fn exp(&mut self, expr: &ExpExpr) -> ExprDesc {
    unimplemented!()
  }
}

impl<'a> TreeWalkCodeGenerator<'a> {
  /// ```bnf
  /// <statement>
  ///   -> <id> := <exp>
  ///   | if <l-exp> then <statement> [else <statement>]
  ///   | while <l-exp> do <statement>
  ///   | call <id> ([<exp> {, <exp>}])
  ///   | <body>
  ///   | read (<id> {, <id>})
  ///   | write (<exp> {, <exp>})
  fn stat(&mut self, expr: &StatementExpr) {
    match expr {
      StatementExpr::Id { id, exp } => todo!(),
      StatementExpr::If { .. } => self.if_stat(expr),
      StatementExpr::While { .. } => self.while_stat(expr),
      StatementExpr::Call { id, args } => todo!(),
      StatementExpr::Body { body } => todo!(),
      StatementExpr::Read { id_list } => todo!(),
      StatementExpr::Write { exps } => todo!(),
    }
  }

  /// ```bnf
  /// <statement> -> <body>
  fn body_stat(&mut self) {}

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

impl<'a> TreeWalkCodeGenerator<'a> {
  // match the name as `local`, `upvalue`, or `global`
  fn simple_name(&mut self, name: String) -> ExprDesc {
    let mut level_iter = self.ctx.levels.iter_mut().rev();

    // search from locals and upvalues in current level
    let level = level_iter.next().unwrap();
    // search from `locals`
    if let Some(i) = level.locals.iter().rposition(|v| v.0 == name) {
      // search reversely, so new variable covers old one with same name
      return ExprDesc::Local(i);
    }
    // search from `upvalues`
    if let Some(i) = level.upvalues.iter().position(|v| v.0 == name) {
      return ExprDesc::UpValue(i);
    }

    // search in upper levels
    for (depth, level) in level_iter.enumerate() {
      if let Some(i) = level.locals.iter().rposition(|v| v.0 == name) {
        level.locals[i].1 = true; // mark it referred as upvalue
        return self.create_upvalue(name, UpIndex::Local(i), depth);
      }
      if let Some(i) = level.upvalues.iter().position(|v| v.0 == name) {
        return self.create_upvalue(name, UpIndex::UpValue(i), depth);
      }
    }

    // not matched as local or upvalue, so global variable, by _ENV[name]
    let i_name = self.add_const(name);
    match self.simple_name("_ENV".into()) {
      ExprDesc::Local(i) => ExprDesc::IndexField(i, i_name),
      ExprDesc::UpValue(i) => ExprDesc::IndexUpField(i, i_name),
      _ => panic!("not here"), // because "_ENV" must exist!
    }
  }

  fn create_upvalue(&mut self, name: String, mut up_idx: UpIndex, depth: usize) -> ExprDesc {
    let levels = &mut self.ctx.levels;
    let last = levels.len() - 1;

    // create upvalue in middle levels, if any
    for Level { upvalues, .. } in levels[last - depth..last].iter_mut() {
      upvalues.push((name.clone(), up_idx));
      up_idx = UpIndex::UpValue(upvalues.len() - 1);
    }

    // create upvalue in current level
    let upvalues = &mut levels[last].upvalues;
    upvalues.push((name, up_idx));
    ExprDesc::UpValue(upvalues.len() - 1)
  }
}

impl<'a> TreeWalkCodeGenerator<'a> {
  /// add the value to constants
  fn add_const(&mut self, c: impl Into<Value>) -> usize {
    let c = c.into();
    let constants = &mut self.ar.constants;
    constants
      .iter()
      .position(|v| v.same(&c))
      .unwrap_or_else(|| {
        constants.push(c);
        constants.len() - 1
      })
  }
}

impl<'a> TreeWalkCodeGenerator<'a> {
  /// discharge @desc into the top of stack, if need
  fn discharge_any(&mut self, desc: ExprDesc) -> usize {
    let dst = if let &ExprDesc::Call(i_func, _) = &desc {
      i_func
    } else {
      self.sp
    };
    self.discharge_if_need(dst, desc)
  }

  /// discharge @desc into @dst, if need
  fn discharge_if_need(&mut self, dst: usize, desc: ExprDesc) -> usize {
    if let ExprDesc::Local(i) = desc {
      i // no need
    } else {
      self.discharge(dst, desc);
      dst
    }
  }

  /// discharge @desc into @dst, and update `self.sp = dst+1``
  fn discharge(&mut self, dst: usize, desc: ExprDesc) {
    let code = match desc {
      ExprDesc::Nil => ByteCode::LoadNil(dst as u8, 1),
      ExprDesc::Boolean(b) => ByteCode::LoadBool(dst as u8, b),
      ExprDesc::Integer(i) => {
        if let Ok(i) = i16::try_from(i) {
          ByteCode::LoadInt(dst as u8, i)
        } else {
          ByteCode::LoadConst(dst as u8, self.add_const(i) as u16)
        }
      }
      ExprDesc::String(s) => ByteCode::LoadConst(dst as u8, self.add_const(s) as u16),
      ExprDesc::Local(src) => {
        if dst != src {
          ByteCode::Move(dst as u8, src as u8)
        } else {
          return;
        }
      }
      ExprDesc::UpValue(src) => ByteCode::GetUpvalue(dst as u8, src as u8),
      ExprDesc::VarArgs => ByteCode::VarArgs(dst as u8, 1),
      ExprDesc::Call(i_func, n_arg_plus) => {
        ByteCode::CallSet(dst as u8, i_func as u8, n_arg_plus as u8)
      }
      ExprDesc::UnaryOp { op, operand } => op(dst as u8, operand as u8),
      ExprDesc::BinaryOp {
        op,
        l_operand,
        r_operand,
      } => op(dst as u8, l_operand as u8, r_operand as u8),
      ExprDesc::Test {
        condition,
        true_list,
        false_list,
      } => {
        // fix TestSet list after discharging
        self.discharge(dst, *condition);
        self.fix_test_set_list(true_list, dst);
        self.fix_test_set_list(false_list, dst);
        return;
      }
      ExprDesc::Compare {
        op,
        l_operand,
        r_operand,
        true_list,
        false_list,
      } => {
        self
          .ar
          .byte_codes
          .push(op(l_operand as u8, r_operand as u8, false));

        // terminate false_list to SetFalseSkip
        self.fix_test_list(false_list);
        self.ar.byte_codes.push(ByteCode::SetFalseSkip(dst as u8));

        // terminate true_list to LoadBool (true)
        self.fix_test_list(true_list);
        ByteCode::LoadBool(dst as u8, true)
      }
      _ => todo!(),
    };
    self.ar.byte_codes.push(code);
    self.sp = dst + 1;
  }

  /// For constant types, add @desc to constants
  ///
  /// Otherwise, discharge @desc into stack
  fn discharge_const(&mut self, desc: ExprDesc) -> ConstStack {
    match desc {
      ExprDesc::Nil => ConstStack::Const(self.add_const(())),
      ExprDesc::Boolean(b) => ConstStack::Const(self.add_const(b)),
      ExprDesc::Integer(i) => ConstStack::Const(self.add_const(i)),
      ExprDesc::String(s) => ConstStack::Const(self.add_const(s)),
      ExprDesc::Function(f) => ConstStack::Const(f),
      _ => todo!(),
    }
  }
}

impl<'a> TreeWalkCodeGenerator<'a> {
  /// Generate a TestOrJump: test @condition or jump to somewhere unknown.
  ///
  /// Link the new code to previous false-list if any.
  ///
  /// Close true_list if any.
  ///
  /// Return false_list to be fixed later in fix_test_list()
  fn test_or_jump(&mut self, condition: ExprDesc) -> Vec<usize> {
    let (code, true_list, mut false_list) = match condition {
      ExprDesc::Boolean(true) | ExprDesc::Integer(_) | ExprDesc::String(_) => {
        // always true, no need to test or jump
        return Vec::new();
      }
      ExprDesc::Compare {
        op,
        l_operand,
        r_operand,
        true_list,
        false_list,
      } => {
        self
          .ar
          .byte_codes
          .push(op(l_operand as u8, r_operand as u8, true));
        (ByteCode::Jump(0), Some(true_list), false_list)
      }
      _ => {
        let i_condition = self.discharge_any(condition);
        (ByteCode::TestOrJump(i_condition as u8, 0), None, vec![])
      }
    };
    unimplemented!()
  }

  /// fix TestAndJump/TestOrJump list to jump to current place
  fn fix_test_list(&mut self, list: Vec<usize>) {
    let here = self.ar.byte_codes.len();
    self.fix_test_list_to(list, here);
  }

  /// fix TestAndJump/TestOrJump list to jump to $to
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

  // fix TestAndJump/TestOrJump list to TestAndSetJump/TestOrSetJump
  fn fix_test_set_list(&mut self, list: Vec<usize>, dst: usize) {
    let here = self.ar.byte_codes.len();
    let dst = dst as u8;
    for i in list.into_iter() {
      let jmp = here - i - 1; // should not be negative
      let code = match self.ar.byte_codes[i] {
        ByteCode::Jump(0) => ByteCode::Jump(jmp as i16),
        ByteCode::TestOrJump(i_condition, 0) => {
          if i_condition == dst {
            ByteCode::TestOrJump(i_condition, jmp as i16)
          } else {
            ByteCode::TestOrSetJump(dst, i_condition, jmp as u8)
          }
        }
        ByteCode::TestAndJump(i_condition, 0) => {
          if i_condition == dst {
            ByteCode::TestAndJump(i_condition, jmp as i16)
          } else {
            ByteCode::TestAndSetJump(dst, i_condition, jmp as u8)
          }
        }
        _ => panic!("invalid Test"),
      };
      self.ar.byte_codes[i] = code;
    }
  }
}

impl<'a> TreeWalkCodeGenerator<'a> {
  /// generate Close if any local variable in [from..] referred as upvalue
  fn local_check_close(&mut self, from: usize) {
    let mut vars = self.ctx.levels.last().unwrap().locals[from..].iter();
    if vars.any(|(_, referred_as_up_value)| *referred_as_up_value) {
      self.ar.byte_codes.push(ByteCode::Close(from as u8));
    }
  }

  fn local_expire(&mut self, from: usize) {
    // drop locals
    let mut vars = self.ctx.levels.last_mut().unwrap().locals.drain(from..);
    // generate Close if any dropped local variable referred as upvalue
    if vars.any(|(name, referred_as_up_value)| referred_as_up_value) {
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

fn chunk<T: Into<AstExpr> + Clone>(
  ctx: &mut CodegenContext,
  has_var_args: bool,
  params: Vec<String>,
  ast_entry: &T,
) -> ActivationRecord {
  // prepare
  let ar = ActivationRecord {
    has_var_args,
    n_param: params.len(),
    ..Default::default()
  };
  ctx.levels.push(Level {
    locals: params.into_iter().map(|p| (p, false)).collect(),
    upvalues: vec![],
  });
  let generator = TreeWalkCodeGenerator {
    ctx,
    curr_ast: ast_entry.to_owned().into(),
    ar,
    sp: 0,
  };

  // generate
  todo!();

  // clear
  let TreeWalkCodeGenerator { mut ar, ctx, .. } = generator;
  let level = ctx.levels.pop().unwrap();
  ar.up_indexes = level.upvalues.into_iter().map(|(_, i)| i).collect();

  ar
}

pub fn load(lexer_ref: &Lexer, ast_entry: Box<ProgramExpr>) -> ActivationRecord {
  let mut ctx = CodegenContext { levels: vec![] };
  chunk(&mut ctx, false, vec!["_ENV".into()], &ast_entry)
}
