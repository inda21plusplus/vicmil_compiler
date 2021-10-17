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

fn print_parse_tree(parsed_tree: &OperatorElement, depth: u32) {
    match parsed_tree {
        OperatorElement::ExpressionTree(arg) => {
            print_parse_tree(&arg.arg1, depth + 1);
            print_parse_tree(&arg.arg2, depth + 2);
            println!("${}: ${} {} ${}", depth, depth+1, arg.operator.text, depth+2);
        }
        OperatorElement::Token(arg) => {
            println!("let ${} be {}", depth, arg.text);
        }
        OperatorElement::Parenthesis(arg) => {
            print_parse_tree(&arg.arg, depth);
        }
        OperatorElement::IdentifierOperation(arg) => {
            print_parse_tree(&arg.arg2, depth);
            print_parse_tree(&arg.operator, depth+1);
            println!("call ${} as ${}", depth, depth + 1);
            return;
        }
    }
}

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
        let parsed_tree =  parsed_tree.unwrap().unwrap();
        print_parse_tree(&parsed_tree, 0);
        println!("{}",parsed_tree.to_string());
    }
    //println!("{}", data);
    println!("program ended");
}