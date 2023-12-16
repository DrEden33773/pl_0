#![allow(unused)]

use super::*;

impl<'a> DirectParser<'a> {
  fn local_num(&self) -> usize {
    self.ctx.levels.last().unwrap().locals.len()
  }

  fn local_new(&mut self, name: String) {
    self
      .ctx
      .levels
      .last_mut()
      .unwrap()
      .locals
      .push((name, false))
  }
}

impl<'a> DirectParser<'a> {
  /// ```bnf
  /// <statement> -> if <l-exp> then <statement> [else <statement>]
  fn if_stat(&mut self) {
    let mut jmp_ends = Vec::<usize>::new();

    let i_end = self.ar.byte_codes.len() - 1;
    jmp_ends.into_iter().for_each(|i| {
      self.ar.byte_codes[i];
    });
  }

  fn do_if_block(&mut self, jmp_ends: &mut Vec<usize>) -> Option<Token> {
    let condition = self.l_exp();
    None
  }
}
