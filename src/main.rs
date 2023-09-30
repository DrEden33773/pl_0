use once_cell::sync::Lazy;
use pl_0::lexer::Lexer;
use project_root::get_project_root;
use std::{env::args, fs::File, io::Read};

static ARGS: Lazy<Vec<String>> = Lazy::new(|| args().collect::<Vec<_>>());
static PROJECT_ROOT: Lazy<String> =
  Lazy::new(|| get_project_root().unwrap().to_str().unwrap().to_string());

fn compile_from_file(src: &str) {
  std::env::set_current_dir(PROJECT_ROOT.as_str()).unwrap();
  let mut string_buf = String::new();
  File::open(src)
    .unwrap()
    .read_to_string(&mut string_buf)
    .unwrap();
  let token_list = Lexer::dbg_one_pass(&string_buf);
  println!("TokenList: {:#?}", token_list);
}

#[allow(unused_variables)]
fn main() {
  if let [_, source, ..] = &ARGS[..] {
    compile_from_file((PROJECT_ROOT.to_string() + source.as_str()).as_str());
  }
  println!("Usage: {} <source_path>", ARGS[0]);
}

#[cfg(test)]
mod demo {
  use super::*;

  #[test]
  fn lexer_demo() {
    let ctx = "
      program OnePlusTwo;
      begin
        var a, b, c;
        a := 1;
        b := 2;
        c := a + b;
        write(c);
      end.
    ";
    let token_list = Lexer::dbg_one_pass(ctx);
    println!("TokenList: {:#?}", token_list);
  }

  #[test]
  fn lexer_should_failed_demo() {
    let ctx = "
      program ChineseProgramming;
      begin
        var 一, 二, 三;
        一 := 1;
        二 := 2;
        三 := 一 + 二;
        write(三);
      end.
    ";
    Lexer::dbg_one_pass(ctx);
  }
}
