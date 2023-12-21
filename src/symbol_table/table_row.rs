use super::sym_type::SymType;

#[derive(Debug, Default, Clone)]
pub struct TableRow {
  pub ty: SymType,
  pub val: i64,
  pub level: usize,
  pub addr: usize,
  pub size: usize,
  pub name: String,
  pub scope_list: Vec<String>,
}

impl TableRow {
  pub fn set_val(&mut self, val: i64) {
    self.val = val;
  }

  pub fn set_size(&mut self, size: usize) {
    self.size = size;
  }
}
