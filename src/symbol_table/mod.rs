use self::{sym_type::SymType, table_row::TableRow};

pub mod sym_type;
pub mod table_row;

#[derive(Debug, Clone, Default)]
pub struct SymTable {
  pub table: Vec<TableRow>,
  pub table_ptr: usize,
}

impl SymTable {
  pub fn try_find_closest_sym(
    &self,
    name: &str,
    curr_level: usize,
    curr_scope_list: &[String],
  ) -> Option<&TableRow> {
    // must `rev()`
    //
    // you should find a symbol with as higher level as you can
    //
    // higher level's symbol always appears later in the linear table
    //
    // another condition: sym.scope_list was totally contained by curr_scope_list
    self.table.iter().rev().find(|&sym| {
      sym.name == name
        && sym.level <= curr_level
        && sym
          .scope_list
          .iter()
          .all(|scope| curr_scope_list.contains(scope))
    })
  }

  pub fn find_closest_sym(
    &self,
    name: &str,
    curr_level: usize,
    curr_scope_list: &[String],
  ) -> &TableRow {
    self
      .try_find_closest_sym(name, curr_level, curr_scope_list)
      .unwrap()
  }

  pub fn get_proc_in_curr_level(&self) -> Option<usize> {
    for i in (0..self.table.len()).rev() {
      if let SymType::Proc = self.table[i].ty {
        return Some(i);
      }
    }
    None
  }

  pub fn is_pre_exists(&self, name: &str, level: usize) -> bool {
    for sym in &self.table {
      if sym.name == name && sym.level <= level {
        return true;
      }
    }
    false
  }

  pub fn is_now_exists(&self, name: &str, level: usize) -> bool {
    for sym in &self.table {
      if sym.name == name && sym.level == level {
        return true;
      }
    }
    false
  }

  pub fn load_const(
    &mut self,
    name: &str,
    level: usize,
    val: i64,
    addr: usize,
    scope_list: Vec<String>,
  ) {
    let value = TableRow {
      ty: SymType::Const,
      val,
      level,
      addr,
      size: 0,
      name: name.to_string(),
      scope_list,
    };
    self.table.push(value);
    self.table_ptr += 1;
  }

  pub fn load_var(&mut self, name: &str, level: usize, addr: usize, scope_list: Vec<String>) {
    let value = TableRow {
      ty: SymType::Var,
      val: 0,
      level,
      addr,
      size: 0,
      name: name.to_string(),
      scope_list,
    };
    self.table.push(value);
    self.table_ptr += 1;
  }

  pub fn load_proc(&mut self, name: &str, level: usize, addr: usize, scope_list: Vec<String>) {
    let value = TableRow {
      ty: SymType::Proc,
      val: 0,
      level,
      addr,
      size: 0,
      name: name.to_string(),
      scope_list,
    };
    self.table.push(value);
    self.table_ptr += 1;
  }
}
