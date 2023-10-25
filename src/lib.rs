use once_cell::sync::Lazy;

pub mod ast;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod util;

pub static SEP: Lazy<String> = Lazy::new(|| "=".repeat(60));
