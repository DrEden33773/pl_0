use std::io::Write;

use crate::pcode::{AllPCode, Pcode, PcodeType};

const STACK_SIZE: usize = 4096;
const SEP: &str = "  ";

/// ## Format
///
/// each data slice:
///
/// (sp) > | DL | SL | RA | ... | data | (top) |
///
/// - DL: Dynamic Link (old_sp)
/// - SL: Static Link (use this to find direct outer level's DL)
/// - RA: Return Address (next_ip)
#[derive(Debug, Clone)]
pub struct VM {
  data: [i64; STACK_SIZE],
  code: AllPCode,
}

impl VM {
  pub fn new(code: AllPCode) -> Self {
    Self {
      data: [0; STACK_SIZE],
      code,
    }
  }

  fn get_base(&self, base: usize, level: usize) -> usize {
    let mut upper_base = base;
    for _ in 0..level {
      upper_base = self.data[upper_base + 1] as usize;
    }
    upper_base
  }

  /// ## Format
  ///
  /// each data slice:
  ///
  /// (sp) > | DL | SL | RA | ... | data | (top) |
  ///
  /// - DL: Dynamic Link (old_sp)
  /// - SL: Static Link (use this to find direct outer level's DL)
  /// - RA: Return Address (next_ip)
  pub fn interpret(&mut self) {
    let mut pc = 0;
    let mut base = 0;
    let mut top = 0;
    loop {
      let inst: Pcode = self.code.pcode_list[pc];
      pc += 1;
      match inst.f {
        PcodeType::NIL => panic!("invalid instruction"),
        PcodeType::LIT => {
          self.data[top] = inst.a;
          top += 1;
        }
        PcodeType::OPR => match inst.a as usize {
          0 => {
            top = base;
            pc = self.data[top + 2] as usize;
            base = self.data[top] as usize;
          }
          1 => self.data[top - 1] = -self.data[top - 1],
          2 => {
            top -= 1;
            self.data[top - 1] += self.data[top];
          }
          3 => {
            top -= 1;
            self.data[top - 1] -= self.data[top];
          }
          4 => {
            top -= 1;
            self.data[top - 1] *= self.data[top];
          }
          5 => {
            top -= 1;
            self.data[top - 1] /= self.data[top];
          }
          6 => self.data[top - 1] %= 2,
          7 => continue,
          8 => {
            top -= 1;
            self.data[top - 1] = (self.data[top - 1] == self.data[top]) as i64;
          }
          9 => {
            top -= 1;
            self.data[top - 1] = (self.data[top - 1] != self.data[top]) as i64;
          }
          10 => {
            top -= 1;
            self.data[top - 1] = (self.data[top - 1] < self.data[top]) as i64;
          }
          11 => {
            top -= 1;
            self.data[top - 1] = (self.data[top - 1] >= self.data[top]) as i64;
          }
          12 => {
            top -= 1;
            self.data[top - 1] = (self.data[top - 1] > self.data[top]) as i64;
          }
          13 => {
            top -= 1;
            self.data[top - 1] = (self.data[top - 1] <= self.data[top]) as i64;
          }
          14 => {
            print!("==> {}{}", self.data[top - 1], SEP);
            // top -= 1;
          }
          15 => println!(),
          16 => {
            print!("<== Please input: ");
            // immediate output
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input
              .trim()
              .split(char::is_whitespace)
              .nth(0)
              .unwrap()
              .parse::<i64>()
              .unwrap();
            self.data[top] = input;
            top += 1;
          }
          _ => panic!("invalid operator"),
        },
        PcodeType::LOD => {
          self.data[top] = self.data[self.get_base(base, inst.l) + inst.a as usize];
          top += 1;
        }
        PcodeType::STO => {
          top -= 1;
          self.data[self.get_base(base, inst.l) + inst.a as usize] = self.data[top];
        }
        PcodeType::CAL => {
          // new: SL
          self.data[top] = base as i64;
          // new: DL
          self.data[top + 1] = self.get_base(base, inst.l) as i64;
          // new: RA(ip)
          self.data[top + 2] = pc as i64;
          base = top;
          pc = inst.a as usize;
        }
        PcodeType::INT => top += inst.a as usize,
        PcodeType::JMP => pc = inst.a as usize,
        PcodeType::JPC => {
          if self.data[top - 1] != 0 {
            pc = inst.a as usize
          }
          // top -= 1;
        }
        PcodeType::RED => {
          print!("<== Please input: ");
          // immediate output
          std::io::stdout().flush().unwrap();
          let mut input = String::new();
          std::io::stdin().read_line(&mut input).unwrap();
          let input = input
            .trim()
            .split(char::is_whitespace)
            .nth(0)
            .unwrap()
            .parse::<i64>()
            .unwrap();
          self.data[self.get_base(base, inst.l) + inst.a as usize] = input;
        }
        PcodeType::WRT => {
          print!("==> {}{}", self.data[top - 1], SEP);
          // top -= 1;
        }
      }
      if pc == 0 {
        break;
      }
    }
  }
}

impl Default for VM {
  fn default() -> Self {
    Self {
      data: [0; STACK_SIZE],
      code: AllPCode::default(),
    }
  }
}
