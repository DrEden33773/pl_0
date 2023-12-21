#![allow(dead_code)]

use crate::{
  ast::{
    AopExpr, BlockExpr, BodyExpr, ConstDeclExpr, ConstExpr, ExpExpr, FactorExpr, LExpExpr, LopExpr,
    MopExpr, ProcExpr, ProgramExpr, StatementExpr, TermExpr, VarDeclExpr,
  },
  error::error_builder::CompileErrorBuilder,
  pcode::{AllPCode, PcodeType},
  symbol_table::{sym_type::SymType, SymTable},
  SEP,
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

impl Translator {
  pub fn show_sym_table(&self) {
    println!("Symbol Table:");
    println!("{}", SEP.as_str());
    println!(
      "{:>10} | {:<6} | {:<4} | {:<6} | {:<4} | {:<4}",
      "name", "type", "val", "level", "addr", "size"
    );
    println!("{}", SEP.as_str());
    self.sym_table.table.iter().for_each(|sym| {
      println!(
        "{:>10} | {:<6} | {:<4} | {:<6} | {:<4} | {:<4}",
        sym.name,
        sym.ty.to_string(),
        sym.val.to_string(),
        sym.level.to_string(),
        sym.addr.to_string(),
        sym.size.to_string()
      );
    });
    println!("{}", SEP.as_str());
    println!();
  }
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
  pub fn translate(&mut self, entry: &ProgramExpr) -> AllPCode {
    self.program(entry);
    if !self.has_error {
      self.pcode.to_owned()
    } else {
      panic!("|> Errors above occurred (during `translation/codegen`), compiling stopped ... <|\n")
    }
  }
}

impl Translator {
  fn program(&mut self, expr: &ProgramExpr) {
    self.block(&expr.block);
  }

  /// temporarily not support for non-zero-arg-proc
  fn block(&mut self, expr: &BlockExpr) {
    // tmp
    let old_addr = self.addr;

    // init curr level
    let start = self.sym_table.table_ptr;
    let mut pos = 0;
    self.addr = 3; // DL - SL - RA

    // if not main
    if start != 0 {
      pos = self.sym_table.get_proc_in_curr_level().unwrap();

      // hold the args
      self.addr += self.sym_table.table[pos].size;
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
      // retract level back after exit a proc
      self.level -= 1;
    }

    // if not main
    if start != 0 {
      // use STA to load params immediately (data stack, reversed order)
      for i in 1..=self.sym_table.table[pos].size {
        self.pcode.gen(
          PcodeType::STA,
          i,
          (3 + self.sym_table.table[pos].size - i) as i64,
        );
      }
    }

    // fix jmp
    let fixed_a = self.pcode.get_pcode_ptr() as i64;
    self.pcode.pcode_list[tmp_pcode_ptr].set_a(fixed_a);

    // allocate
    self.pcode.gen(PcodeType::INT, 0, self.addr as i64);

    // if not main
    if start != 0 {
      // set curr_proc's sym_value into curr_proc's begin pos
      let val = self.pcode.get_pcode_ptr() - 1 - self.sym_table.table[pos].size;
      self.sym_table.table[pos].set_val(val as i64);
    }

    // call body
    self.body(&expr.body);

    // end of procedure
    self.pcode.gen(PcodeType::OPR, 0, 0);

    self.addr = old_addr;
  }

  /// temporarily not support for non-zero-arg-proc
  fn procedure(&mut self, expr: &ProcExpr) {
    // tmp
    let mut args_count = 0;

    // name
    let name = expr.id.as_ref().0.to_owned();

    // duplicate-definition
    if self.sym_table.is_now_exists(&name, self.level) {
      self.has_error = true;
      CompileErrorBuilder::from(expr.id.as_ref().1)
        .with_info(format!("`{}` is defined before", name))
        .build()
        .show();
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
      self.sym_table.load_var(&id, self.level, args_count + 3);
      args_count += 1;
      self.sym_table.table[proc_pos].set_size(args_count);
    }

    // block
    self.block(&expr.block);

    // procs
    for proc_expr in &expr.procs {
      self.level -= 1;
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
          CompileErrorBuilder::from(id.as_ref().1)
            .with_info(format!("`{}` is undefined", name))
            .build()
            .show();
          return;
        }

        // assign to non-var
        let tmp_sym = self
          .sym_table
          .find_closest_sym(&name, self.level)
          .to_owned();
        if !matches!(tmp_sym.ty, SymType::Var) {
          self.has_error = true;
          CompileErrorBuilder::from(id.as_ref().1)
            .with_info(format!("`{}` is not a variable", name))
            .build()
            .show();
          return;
        }

        // eval expression
        self.exp(exp);

        // STO (store)
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
        self.pcode.gen(PcodeType::JPC, 0, 0);
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
        let n_args = args.len();
        let name = id.as_ref().0.to_owned();

        // undefined
        if !self.sym_table.is_pre_exists(&name, self.level) {
          self.has_error = true;
          CompileErrorBuilder::from(id.as_ref().1)
            .with_info(format!("`{}` is undefined", name))
            .build()
            .show();
          return;
        }

        let tmp_sym = self
          .sym_table
          .find_closest_sym(&name, self.level)
          .to_owned();
        // call non-proc
        if !matches!(tmp_sym.ty, SymType::Proc) {
          self.has_error = true;
          CompileErrorBuilder::from(id.as_ref().1)
            .with_info(format!("`{}` is not a procedure", name))
            .build()
            .show();
          return;
        }
        // unmatchable n_args
        if tmp_sym.size != n_args {
          self.has_error = true;
          CompileErrorBuilder::from(id.as_ref().1)
            .with_info(format!(
              "`{}` expects {} args, but received {}",
              name, tmp_sym.size, n_args
            ))
            .build()
            .show();
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

          // undefined
          if !self.sym_table.is_pre_exists(&name, self.level) {
            self.has_error = true;
            CompileErrorBuilder::from(id.as_ref().1)
              .with_info(format!("`{}` is undefined", name))
              .build()
              .show();
            return;
          }

          let tmp_sym = self
            .sym_table
            .find_closest_sym(&name, self.level)
            .to_owned();
          // read to non-var
          if !matches!(tmp_sym.ty, SymType::Var) {
            self.has_error = true;
            CompileErrorBuilder::from(id.as_ref().1)
              .with_info(format!("`{}` is not a variable", name))
              .build()
              .show();
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
        CompileErrorBuilder::from(exp.id.as_ref().1)
          .with_info(format!("`{}` is defined before", id))
          .build()
          .show();
      } else {
        self.sym_table.load_const(&id, self.level, val, self.addr);
      }
    }
  }

  fn var_decl(&mut self, expr: &VarDeclExpr) {
    let id_list = &expr.id_list;
    // for each id in id_list, you should consider the updating of addr
    for id_exp in id_list {
      let id = id_exp.as_ref().0.to_owned();
      if self.sym_table.is_now_exists(&id, self.level) {
        self.has_error = true;
        CompileErrorBuilder::from(id_exp.as_ref().1)
          .with_info(format!("`{}` is defined before", id))
          .build()
          .show();
        continue;
      } else {
        self.sym_table.load_var(&id, self.level, self.addr);
        // update addr
        self.addr += self.addr_increment;
      }
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
          let tmp_sym = self.sym_table.find_closest_sym(&id, self.level);
          match tmp_sym.ty {
            SymType::Nil => {
              self.has_error = true;
              CompileErrorBuilder::from(expr.1)
                .with_info(format!(
                  "`{}` has an non-r-value type `nil` (only `var` or `const` appears after `:=`)",
                  id
                ))
                .build()
                .show();
            }
            SymType::Const => self.pcode.gen(PcodeType::LIT, 0, tmp_sym.val),
            SymType::Var => {
              assert!(self.level >= tmp_sym.level);
              self.pcode.gen(
                PcodeType::LOD,
                self.level - tmp_sym.level,
                tmp_sym.addr as i64,
              )
            }
            SymType::Proc => {
              self.has_error = true;
              CompileErrorBuilder::from(expr.1)
                .with_info(format!(
                  "`{}` has an non-r-value type `procedure` (only `var` or `const` appears after `:=`)",
                   id
                ))
                .build()
                .show();
            }
          }
        } else {
          self.has_error = true;
          CompileErrorBuilder::from(expr.1)
            .with_info(format!("`{}` is undefined", id))
            .build()
            .show();
        }
      }
    }
  }

  fn lop(&mut self, expr: &LopExpr) -> LopExpr {
    expr.to_owned()
  }
}
