use once_cell::sync::Lazy;
use pl_0::{lexer::Lexer, parser::Parser};
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
  let mut parser = Parser::new(&string_buf);
  parser.parse();
  let token_list = Lexer::dbg_one_pass(&string_buf);
  println!("TokenList: {:?}", token_list);
}

fn main() {
  if let [_, source, ..] = &ARGS[..] {
    compile_from_file((PROJECT_ROOT.to_string() + source.as_str()).as_str());
  } else {
    println!("Usage: {} <source_path>", ARGS[0]);
  }
}

#[cfg(test)]
mod demo {
  use super::*;

  fn file_to_string(filename: String) -> String {
    let mut string_buf = String::new();
    File::open(filename)
      .unwrap()
      .read_to_string(&mut string_buf)
      .unwrap();
    string_buf
  }

  #[test]
  fn lexer_demo() {
    let ctx = &file_to_string(PROJECT_ROOT.to_string() + "/examples/lexer/one_plus_two.pas");
    let token_list = Lexer::dbg_one_pass(ctx);
    println!("TokenList: {:#?}", token_list);
  }

  #[test]
  #[should_panic]
  fn chinese_character_demo() {
    Lexer::dbg_one_pass(&file_to_string(
      PROJECT_ROOT.to_string() + "/examples/lexer/chinese_programming.pas",
    ));
  }

  #[test]
  #[should_panic]
  fn single_colon_demo() {
    Lexer::dbg_one_pass(&file_to_string(
      PROJECT_ROOT.to_string() + "/examples/lexer/single_colon.pas",
    ));
  }

  #[test]
  #[should_panic]
  fn unsupported_ascii_char_demo() {
    Lexer::dbg_one_pass(&file_to_string(
      PROJECT_ROOT.to_string() + "/examples/lexer/unsupported_ascii_char.pas",
    ));
  }

  #[test]
  #[should_panic]
  fn malformed_char_demo() {
    Lexer::dbg_one_pass(&file_to_string(
      PROJECT_ROOT.to_string() + "/examples/lexer/japanese_programming.pas",
    ));
  }
}
