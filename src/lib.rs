use once_cell::sync::Lazy;

pub mod bytecode;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod pest_parser;
pub mod sat;
pub mod util;
pub mod vm;

pub static SEP: Lazy<String> = Lazy::new(|| "=".repeat(60));
