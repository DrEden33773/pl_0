#![allow(dead_code)]

use crate::{
  ast::{
    AopExpr, BlockExpr, BodyExpr, ConstDeclExpr, ConstExpr, ExpExpr, FactorExpr, LExpExpr, LopExpr,
    MopExpr, ProcExpr, ProgramExpr, StatementExpr, TermExpr, VarDeclExpr,
  },
  pcode::{AllPCode, PcodeType},
  symbol_table::{sym_type::SymType, SymTable},
};

#[derive(Debug, Clone)]
pub struct Translator {
  pub pcode: AllPCode,
  pub sym_table: SymTable,
  pub has_error: bool,
  pub level: usize,
  pub addr: usize,
  pub addr_increment: usize,
}

impl Default for Translator {
  fn default() -> Self {
    Self {
      pcode: Default::default(),
      sym_table: Default::default(),
      has_error: Default::default(),
      level: Default::default(),
      addr: Default::default(),
      addr_increment: 1,
    }
  }
}

impl Translator {
  pub fn translate(mut self, entry: &ProgramExpr) -> AllPCode {
    self.program(entry);
    if !self.has_error {
      self.pcode
    } else {
      panic!("|> Errors above occurred (during `translation/codegen`), compiling stopped ... <|\n")
    }
  }
}

impl Translator {
  fn program(&mut self, expr: &ProgramExpr) {
    self.block(&expr.block);
  }

  fn block(&mut self, expr: &BlockExpr) {
    // tmp
    let addr_cp = self.addr;

    // init curr level
    let start = self.sym_table.table_ptr;
    let mut pos = 0;
    self.addr = 3; // DL - SL - RA
    if start > 0 {
      pos = self.sym_table.get_proc_in_curr_level().unwrap();
    }

    // (jmp, 0, 0)
    let tmp_pcode_ptr = self.pcode.get_pcode_ptr();
    self.pcode.gen(PcodeType::JMP, 0, 0);

    if let Some(expr) = &expr.const_decl {
      self.const_decl(expr);
    }
    if let Some(expr) = &expr.var_decl {
      self.var_decl(expr);
    }
    if let Some(expr) = &expr.proc {
      self.procedure(expr);
    }

    // fix jmp
    self.pcode.pcode_list[tmp_pcode_ptr].a = self.pcode.get_pcode_ptr() as i64;
    // allocate
    self.pcode.gen(PcodeType::INT, 0, self.addr as i64);
    // if not main
    if start != 0 {
      let val = self.pcode.get_pcode_ptr() - 1 - self.sym_table.table[pos].size;
      self.sym_table.table[pos].set_val(val as i64);
    }

    self.body(&expr.body);

    // end of procedure
    self.pcode.gen(PcodeType::OPR, 0, 0);

    self.addr = addr_cp;
  }

  fn procedure(&mut self, expr: &ProcExpr) {
    // tmp
    let mut arg_count = 0;

    // name
    let name = expr.id.as_ref().0.to_owned();
    if self.sym_table.is_now_exists(&name, self.level) {
      self.has_error = true;
      todo!("Error: {} is defined before", name);
      return;
    }

    let proc_pos = self.sym_table.table_ptr;
    self.sym_table.load_proc(&name, self.level, self.addr);
    self.addr += self.addr_increment;
    self.level += 1; // update level

    // args
    for arg in &expr.args {
      let id = arg.as_ref().0.to_owned();
      // +3 :: DL - SL - RA
      self.sym_table.load_var(&id, self.level, self.addr + 3);
      self.addr += self.addr_increment;
      arg_count += 1;
      self.sym_table.table[proc_pos].set_size(arg_count);
    }

    // block
    self.block(&expr.block);

    // procs
    for proc_expr in &expr.procs {
      self.level -= 1; // same level, so sub 1 ahead, add 1 later
      self.procedure(proc_expr);
    }
  }

  fn body(&mut self, expr: &BodyExpr) {
    for e in expr.statements.iter() {
      self.statement(e);
    }
  }
}

impl Translator {
  fn statement(&mut self, expr: &StatementExpr) {
    match expr {
      StatementExpr::Id { id, exp } => {
        let name = id.as_ref().0.to_owned();

        // undefined
        if !self.sym_table.is_pre_exists(&name, self.level) {
          self.has_error = true;
          todo!("Error: {} is not defined", name);
          return;
        }

        // assign to non-var
        let tmp_sym = self.sym_table.find_symbol(&name).to_owned();
        if !matches!(tmp_sym.ty, SymType::Var) {
          self.has_error = true;
          todo!("Error: {} is not `var`", name);
          return;
        }

        // eval expression
        self.exp(exp);
        self.pcode.gen(
          PcodeType::STO,
          self.level - tmp_sym.level,
          tmp_sym.addr as i64,
        );
      }
      StatementExpr::If {
        l_exp,
        then_statement,
        else_statement,
      } => {
        // condition
        self.l_exp(l_exp);

        // then
        let pos1 = self.pcode.get_pcode_ptr();
        self.pcode.gen(PcodeType::JMP, 0, 0);
        self.statement(then_statement);
        let pos2 = self.pcode.get_pcode_ptr();
        self.pcode.gen(PcodeType::JMP, 0, 0);
        let fixed_a = self.pcode.get_pcode_ptr() as i64;
        self.pcode.pcode_list[pos1].set_a(fixed_a);
        self.pcode.pcode_list[pos2].set_a(fixed_a);

        // else
        if let Some(else_statement) = else_statement {
          self.statement(else_statement);
          let fixed_a = self.pcode.get_pcode_ptr() as i64;
          self.pcode.pcode_list[pos2].set_a(fixed_a);
        }
      }
      StatementExpr::While { l_exp, statement } => {
        let pos1 = self.pcode.get_pcode_ptr();

        // condition
        self.l_exp(l_exp);

        // do(statement)
        let pos2 = self.pcode.get_pcode_ptr();
        self.pcode.gen(PcodeType::JPC, 0, 0); // jump out if not condition
        self.statement(statement);
        self.pcode.gen(PcodeType::JMP, 0, pos1 as i64); // jump back to while
        let fixed_a = self.pcode.get_pcode_ptr() as i64;
        self.pcode.pcode_list[pos2].set_a(fixed_a);
      }
      StatementExpr::Call { id, args } => {
        let n_arg = args.len();
        let name = id.as_ref().0.to_owned();

        // undefined
        if !self.sym_table.is_pre_exists(&name, self.level) {
          self.has_error = true;
          todo!("Error: {} is not defined", name);
          return;
        }

        let tmp_sym = self.sym_table.find_symbol(&name).to_owned();
        // call non-proc
        if !matches!(tmp_sym.ty, SymType::Proc) {
          self.has_error = true;
          todo!("Error: {} is not `proc`", name);
          return;
        }
        // unmatchable n_arg
        if tmp_sym.size != n_arg {
          self.has_error = true;
          todo!("Error: {}'s args is not matched", name);
          return;
        }

        for arg in args {
          // eval-exp
          self.exp(arg);
        }

        // CAL
        self
          .pcode
          .gen(PcodeType::CAL, self.level - tmp_sym.level, tmp_sym.val);
      }
      StatementExpr::Body { body } => self.body(body),
      StatementExpr::Read { id_list } => {
        for id in id_list {
          let name = id.as_ref().0.to_owned();
          if !self.sym_table.is_pre_exists(&name, self.level) {
            self.has_error = true;
            todo!("Error: {} is not defined", name);
            return;
          }
          let tmp_sym = self.sym_table.find_symbol(&name).to_owned();
          if !matches!(tmp_sym.ty, SymType::Var) {
            self.has_error = true;
            todo!("Error: {} is not `var`", name);
            return;
          }
          self.pcode.gen(PcodeType::OPR, 0, 16);
          // must gen SPO, because `read` will change sp
          self.pcode.gen(
            PcodeType::STO,
            self.level - tmp_sym.level,
            tmp_sym.addr as i64,
          );
        }
      }
      StatementExpr::Write { exps } => {
        for exp in exps {
          self.exp(exp);
          self.pcode.gen(PcodeType::OPR, 0, 14);
        }
        // println
        self.pcode.gen(PcodeType::OPR, 0, 15);
      }
    }
  }
}

impl Translator {
  fn const_decl(&mut self, expr: &ConstDeclExpr) {
    self.my_const(&expr.constants);
  }

  fn my_const(&mut self, expr: &[Box<ConstExpr>]) {
    for exp in expr {
      let id = exp.id.as_ref().0.to_owned();
      let val = exp.integer.as_ref().0;
      if self.sym_table.is_now_exists(&id, self.level) {
        self.has_error = true;
        todo!("Error: {} is defined before", id)
      } else {
        self.sym_table.load_const(&id, self.level, val, self.addr);
      }
    }
  }

  fn var_decl(&mut self, expr: &VarDeclExpr) {
    let id_list = &expr.id_list;
    // for each id in id_list, you should consider the updating of addr
    for id in id_list {
      let id = id.as_ref().0.to_owned();
      if self.sym_table.is_now_exists(&id, self.level) {
        self.has_error = true;
        todo!("Error: {} is defined before", id)
      } else {
        self.sym_table.load_var(&id, self.level, self.addr);
      }
      // update addr
      self.addr += self.addr_increment;
    }
  }
}

impl Translator {
  fn l_exp(&mut self, expr: &LExpExpr) {
    match expr {
      LExpExpr::Exp { l_exp, lop, r_exp } => {
        self.exp(l_exp);
        let lop = self.lop(lop);
        self.exp(r_exp);
        match lop {
          LopExpr::Eq(_) => self.pcode.gen(PcodeType::OPR, 0, 8),
          LopExpr::Ne(_) => self.pcode.gen(PcodeType::OPR, 0, 9),
          LopExpr::Lt(_) => self.pcode.gen(PcodeType::OPR, 0, 10),
          LopExpr::Ge(_) => self.pcode.gen(PcodeType::OPR, 0, 11),
          LopExpr::Gt(_) => self.pcode.gen(PcodeType::OPR, 0, 12),
          LopExpr::Le(_) => self.pcode.gen(PcodeType::OPR, 0, 13),
        }
      }
      LExpExpr::Odd { exp } => {
        // TODO: figure out the witch should be executed first
        self.exp(exp);
        self.pcode.gen(PcodeType::OPR, 0, 6);
      }
    }
  }

  fn exp(&mut self, expr: &ExpExpr) {
    self.term(&expr.term);
    if expr.is_negative {
      // negative
      self.pcode.gen(PcodeType::OPR, 0, 1);
    }
    for (aop, term) in &expr.aop_terms {
      self.term(term);
      match aop.as_ref() {
        // add
        AopExpr::Add(_) => self.pcode.gen(PcodeType::OPR, 0, 2),
        // sub
        AopExpr::Sub(_) => self.pcode.gen(PcodeType::OPR, 0, 3),
      }
    }
  }

  fn term(&mut self, expr: &TermExpr) {
    self.factor(&expr.factor);
    for (mop, factor) in &expr.mop_factors {
      self.factor(factor);
      match mop.as_ref() {
        // mul
        MopExpr::Mul(_) => self.pcode.gen(PcodeType::OPR, 0, 4),
        // div
        MopExpr::Div(_) => self.pcode.gen(PcodeType::OPR, 0, 5),
      }
    }
  }

  fn factor(&mut self, expr: &FactorExpr) {
    match expr {
      FactorExpr::Integer(expr) => {
        let val = expr.0;
        self.pcode.gen(PcodeType::LIT, 0, val);
      }
      FactorExpr::Exp(expr) => self.exp(expr),
      FactorExpr::Id(expr) => {
        let id = expr.0.to_owned();
        if self.sym_table.is_pre_exists(&id, self.level) {
          let tmp_row = self.sym_table.try_find_symbol(&id).unwrap();
          match tmp_row.ty {
            SymType::Nil => todo!("Error: {} is not `const` / `var`", id),
            SymType::Const => self.pcode.gen(PcodeType::LIT, 0, tmp_row.val),
            SymType::Var => {
              debug_assert!(self.level >= tmp_row.level);
              self.pcode.gen(
                PcodeType::LOD,
                self.level - tmp_row.level,
                tmp_row.addr as i64,
              )
            }
            SymType::Proc => todo!("Error: {} is not `const` / `var`", id),
          }
        } else {
          self.has_error = true;
          todo!("Error: {} is not defined", id)
        }
      }
    }
  }

  fn lop(&mut self, expr: &LopExpr) -> LopExpr {
    expr.to_owned()
  }
}
