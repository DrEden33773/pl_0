use once_cell::sync::Lazy;

pub mod ast;
pub mod bytecode;
pub mod codegen;
pub mod error;
pub mod lexer;
pub mod optimizer;
pub mod parser;
pub mod pcode;
pub mod pest_parser;
pub mod symbol_table;
pub mod translator;
pub mod util;
pub mod value;
pub mod vm;

pub static SEP: Lazy<String> = Lazy::new(|| "=".repeat(70));
pub static LINE: Lazy<String> = Lazy::new(|| "-".repeat(70));
