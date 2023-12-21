use std::fmt::Display;

use crate::SEP;

#[derive(Debug, Clone, Copy, Default)]
pub enum PcodeType {
  #[default]
  NIL = -1,
  LIT = 0,
  OPR,
  LOD,
  STO,
  STA,
  CAL,
  INT,
  JMP,
  JPC,
  RED,
  WRT,
}

impl Display for PcodeType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // let pcode_type = match self {
    //   Self::NIL => "NIL",
    //   Self::LIT => "LIT",
    //   Self::OPR => "OPR",
    //   Self::LOD => "LOD",
    //   Self::STO => "STO",
    //   Self::CAL => "CAL",
    //   Self::INT => "INT",
    //   Self::JMP => "JMP",
    //   Self::JPC => "JPC",
    //   Self::RED => "RED",
    //   Self::WRT => "WRT",
    // };
    write!(f, "{:?}", self)
  }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Pcode {
  pub f: PcodeType,
  pub l: usize,
  pub a: i64,
}

impl Pcode {
  pub fn set_l(&mut self, l: usize) {
    self.l = l;
  }

  pub fn set_a(&mut self, a: i64) {
    self.a = a;
  }
}

#[derive(Debug, Clone, Default)]
pub struct PCodeManager {
  pub pcode_list: Vec<Pcode>,
}

impl PCodeManager {
  pub fn show_pcode_list(&self) {
    println!();
    println!("PCode list:");
    println!("{}", SEP.as_str());
    for (i, pcode) in self.pcode_list.iter().enumerate() {
      println!("{:4}| {:4} {:4} {:4}", i, pcode.f, pcode.l, pcode.a);
    }
    println!("{}", SEP.as_str());
    println!();
  }

  pub fn get_pcode_ptr(&self) -> usize {
    self.pcode_list.len()
  }
}

impl PCodeManager {
  pub fn gen(&mut self, f: PcodeType, l: usize, a: i64) {
    self.pcode_list.push(Pcode { f, l, a });
  }
}
