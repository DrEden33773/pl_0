use once_cell::sync::Lazy;
use pl_0::{optimizer::AstOptimizer, parser::Parser, translator::Translator, vm::basic::VM};
use project_root::get_project_root;
use std::{env::args, fs::File, io::Read};

static ARGS: Lazy<Vec<String>> = Lazy::new(|| args().collect::<Vec<_>>());
static PROJECT_ROOT: Lazy<String> =
  Lazy::new(|| get_project_root().unwrap().to_str().unwrap().to_string());

fn compile_from_file(src: &str) {
  let mut string_buf = String::new();
  File::open(src)
    .unwrap()
    .read_to_string(&mut string_buf)
    .unwrap();

  let mut parser = Parser::new(&string_buf);
  parser.parse();

  let optimizer = AstOptimizer::new(parser.take_ast_entry());
  let ast_entry = optimizer.optimize();

  let mut translator = Translator::default();
  let code = translator.translate(&ast_entry);
  code.show_pcode_list();
  translator.show_sym_table();

  let mut vm = VM::new(code);
  vm.interpret();
}

fn main() {
  if let [_, source, ..] = &ARGS[..] {
    compile_from_file((PROJECT_ROOT.to_string() + source.as_str()).as_str());
  } else {
    println!("Usage: {} <source_path>", ARGS[0]);
  }
}

#[cfg(test)]
mod dbg {
  use super::*;

  #[test]
  fn dbg() {
    let filename = PROJECT_ROOT.to_string() + "/examples/correct/fib.pas";
    compile_from_file(&filename);
  }
}

#[cfg(test)]
mod demo {
  use super::*;
  use pl_0::lexer::Lexer;

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
  fn parser_demo() {
    let ctx = &file_to_string(PROJECT_ROOT.to_string() + "/examples/lexer/one_plus_two.pas");
    let mut parser = Parser::new(ctx);
    parser.parse();
    parser.show_ast();
  }

  #[test]
  #[should_panic]
  fn chinese_character_demo() {
    let content =
      file_to_string(PROJECT_ROOT.to_string() + "/examples/lexer/chinese_programming.pas");
    let mut parser = Parser::new(&content);
    parser.parse();
    parser.show_ast();
  }

  #[test]
  #[should_panic]
  fn japanese_character_demo() {
    let content =
      file_to_string(PROJECT_ROOT.to_string() + "/examples/lexer/japanese_programming.pas");
    let mut parser = Parser::new(&content);
    parser.parse();
    parser.show_ast();
  }

  #[test]
  #[should_panic]
  fn chinese_in_keyword_demo() {
    let content =
      file_to_string(PROJECT_ROOT.to_string() + "/examples/parser/chinese_in_keyword.pas");
    let mut parser = Parser::new(&content);
    parser.parse();
    parser.show_ast();
  }

  #[test]
  #[should_panic]
  fn single_colon_demo() {
    Parser::new(&file_to_string(
      PROJECT_ROOT.to_string() + "/examples/lexer/single_colon.pas",
    ))
    .parse()
    .show_ast();
  }

  #[test]
  #[should_panic]
  fn unsupported_ascii_char_demo() {
    Parser::new(&file_to_string(
      PROJECT_ROOT.to_string() + "/examples/lexer/unsupported_ascii_char.pas",
    ))
    .parse()
    .show_ast();
  }

  #[test]
  #[should_panic]
  fn malformed_char_demo() {
    Parser::new(&file_to_string(
      PROJECT_ROOT.to_string() + "/examples/lexer/japanese_programming.pas",
    ))
    .parse()
    .show_ast();
  }

  #[test]
  #[should_panic]
  fn losing_prog_id_demo() {
    Parser::new(&file_to_string(
      PROJECT_ROOT.to_string() + "/examples/parser/losing_prog_id.pas",
    ))
    .parse()
    .show_ast();
  }

  #[test]
  #[should_panic]
  fn losing_eqsign_demo() {
    Parser::new(&file_to_string(
      PROJECT_ROOT.to_string() + "/examples/parser/losing_eqsign.pas",
    ))
    .parse()
    .show_ast();
  }

  #[test]
  #[should_panic]
  fn multi_err_demo() {
    Parser::new(&file_to_string(
      PROJECT_ROOT.to_string() + "/examples/parser/multi_err.pas",
    ))
    .parse()
    .show_ast();
  }

  #[test]
  #[should_panic]
  fn wrong_if_demo() {
    Parser::new(&file_to_string(
      PROJECT_ROOT.to_string() + "/examples/parser/wrong_if.pas",
    ))
    .parse()
    .show_ast();
  }
}
