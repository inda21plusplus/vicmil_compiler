pub mod error_handler;
pub mod tokenizer;
pub mod parser;
pub mod test;

pub use crate::error_handler::compiler_error::*;
pub use crate::error_handler::compiler_error::CompilerError::*;
pub use crate::parser::*;
use crate::tokenizer::tokenize;

pub fn test() -> CompilerResult<()> {
    return Err(CompilerError::from("error"));
}

use std::fs;

fn main() {
    println!("staring program");
    let data = fs::read_to_string("code.txt").expect("Unable to read file");
    let mut tokanized_text = tokenize(&data);
    println!("{}", tokanized_text.to_string());
    let parsed_tree = generate_tree(tokanized_text.token_lists.front_mut().unwrap());
    if parsed_tree.is_err() {
        println!("Error!: {}", parsed_tree.err().unwrap().compiler_err_to_string());
    }
    else {
        println!("{}", parsed_tree.unwrap().unwrap().to_string());
    }
    //println!("{}", data);
    println!("program ended");
}