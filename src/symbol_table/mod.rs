use self::{sym_type::SymType, table_row::TableRow};

pub mod sym_type;
pub mod table_row;

pub const SYM_TABLE_MAX_LEN: usize = 10000;

#[derive(Debug, Clone, Default)]
pub struct SymTable {
  pub table: Vec<TableRow>,
  pub table_ptr: usize,
}

impl SymTable {
  pub fn get_symbol(&self, i: usize) -> &TableRow {
    &self.table[i]
  }

  pub fn try_find_symbol(&self, name: &str) -> Option<&TableRow> {
    self.table.iter().find(|&row| row.name == name)
  }

  pub fn find_symbol(&self, name: &str) -> &TableRow {
    self.try_find_symbol(name).unwrap()
  }

  pub fn get_symbol_mut(&mut self, i: usize) -> &mut TableRow {
    &mut self.table[i]
  }

  pub fn get_row_index(&self, name: &str) -> Option<usize> {
    (0..self.table.len()).find(|&i| self.table[i].name == name)
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
    for row in &self.table {
      if row.name == name && row.level <= level {
        return true;
      }
    }
    false
  }

  pub fn is_now_exists(&self, name: &str, level: usize) -> bool {
    for row in &self.table {
      if row.name == name && row.level == level {
        return true;
      }
    }
    false
  }

  pub fn load_const(&mut self, name: &str, level: usize, val: i64, addr: usize) {
    let value = TableRow {
      ty: SymType::Const,
      val,
      level,
      addr,
      size: 4,
      name: name.to_string(),
    };
    self.table.push(value);
    self.table_ptr += 1;
  }

  pub fn load_var(&mut self, name: &str, level: usize, addr: usize) {
    let value = TableRow {
      ty: SymType::Var,
      val: 0,
      level,
      addr,
      size: 0,
      name: name.to_string(),
    };
    self.table.push(value);
    self.table_ptr += 1;
  }

  pub fn load_proc(&mut self, name: &str, level: usize, addr: usize) {
    let value = TableRow {
      ty: SymType::Proc,
      val: 0,
      level,
      addr,
      size: 0,
      name: name.to_string(),
    };
    self.table.push(value);
    self.table_ptr += 1;
  }
}
