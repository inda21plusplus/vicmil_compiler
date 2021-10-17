use crate::tokenizer::*;
use crate::error_handler::compiler_error::*;
use std::borrow::Borrow;
use std::collections::LinkedList;
use std::collections::linked_list::Iter as LinkedListIter;

pub enum OperatorElement {
    ExpressionTree(Box<ExpressionTree>),
    Token(Token),
    OperatorArgs(OperatorArgs)
}

pub struct OperatorArgs {
    arg_list: LinkedList<OperatorElement>,
}
impl OperatorArgs {
    pub fn to_string(&self) -> String {
        let mut text = String::from("(");
        let mut iter_num = 0;
        for i in self.arg_list.iter() {
            if iter_num != 0 {
                text += ", ";
            }
            text += i.to_string().as_str();
            iter_num += 1;
        }
        text += ")";
        return text;
    }
}

impl OperatorElement {
    pub fn to_string(&self) -> String {
        match self {
            OperatorElement::ExpressionTree(tree) => {
                return tree.to_string();
            }
            OperatorElement::Token(token) => {
                return token.text.to_string();
            }
            OperatorElement::OperatorArgs(token) => {
                return token.to_string();
            }
        }
    }
}
pub struct ExpressionTree {
    pub arg1: OperatorElement,
    pub arg2: OperatorElement,
    pub operator: Token
}

impl ExpressionTree {
    pub fn to_string(&self) -> String {
        return "(".to_string()
        + self.arg1.to_string().as_str()
        + " "
        + self.operator.text.as_str() 
        + " "
        + self.arg2.to_string().as_str()
        + ")";
    }
}

// Get arguments for function, list, etc.
pub fn get_arguments_for_func(linked_list_iter: &mut LinkedListIter<Token>) -> CompilerResult<OperatorElement> {
    // Treat first arg as an expression
    // Continue until parenthesis and add arguments to list and return
    return Err(CompilerError::String("Something went wrong when parsing arguments".to_string()));
}

pub fn recursive_generate_tree(token_list: &mut TokenList, arg1_in: Option<OperatorElement>) -> CompilerResult<OperatorElement> {
    // To add: Function call handling

    let arg1: OperatorElement;
    if arg1_in.is_none() {
        // Find first argument
        let element_1 = token_list.tokens.pop_front();
        let element_1 = element_1.unwrap();

        // Parse argument
        if let TokenType::LParen = element_1.token_type {
            // If element_1 is parenthesis, parse that first
            arg1 = recursive_generate_tree(token_list, None)?;
        }
        else if let TokenType::IdentifierOrNumber = element_1.token_type {
            // Just set arg1 to the element_1
            arg1 = OperatorElement::Token(element_1.clone());
        }
        else {
            return Err(CompilerError::from(TokenError::new(element_1.clone(), TokenErrorEnum::UnexpectedType)))
        }
    }
    else {
        arg1 = arg1_in.unwrap();
    }

    // Find first operator
    let operator1 = token_list.tokens.pop_front();
    let operator1 = operator1.unwrap();
    // Make sure it is an operator
    if let TokenType::Operator(_) = operator1.token_type {
    }
    else {
        return Err(CompilerError::from(TokenError::new(operator1.clone(), TokenErrorEnum::ExpectedOperator)))
    }

    // Find second argument
    let element_2 = token_list.tokens.pop_front();
    let element_2 = element_2.unwrap();

    // Parse second argument
    let mut arg2: OperatorElement;
    if let TokenType::LParen = element_2.token_type {
        // If element_2 is parenthesis, parse that first
        arg2 = recursive_generate_tree(token_list, None)?;
    }
    else if let TokenType::IdentifierOrNumber = element_2.token_type {
        // Just set arg2 to the element_2
        arg2 = OperatorElement::Token(element_2.clone());
    }
    else {
        return Err(CompilerError::from(TokenError::new(element_2.clone(), TokenErrorEnum::UnexpectedType)))
    }

    loop {
        // Find operator after first operator
        let operator2 = token_list.tokens.front();
        if operator2.is_none() {
            return Ok(OperatorElement::ExpressionTree( Box::new(ExpressionTree{
                arg1,
                arg2,
                operator: operator1.clone()
            })));
        }
        let operator2 = operator2.unwrap();

        if let TokenType::Operator(_) = operator2.token_type {

        }
        else if let TokenType::LParen = operator2.token_type {
            // Treat it as a function call
        }
        else {
            return Err(CompilerError::from(TokenError::new(operator2.clone(), TokenErrorEnum::ExpectedOperator)))
        }

        // See if operator 2 should execute before operator 1
        match (operator1.token_type, operator2.token_type) {
            (TokenType::Operator(op1), TokenType::Operator(op2)) => {
                if op1 < op2 {
                    // Operator 1 should execute first
                    //prev(linked_list_iter);
                    return Ok(OperatorElement::ExpressionTree( Box::new(ExpressionTree{
                        arg1,
                        arg2,
                        operator: operator1.clone()
                    })));
                }
                else if op1 == op2 {
                    let pass_arg = OperatorElement::ExpressionTree( Box::new(ExpressionTree{
                        arg1,
                        arg2,
                        operator: operator1.clone()
                    }));
                    return recursive_generate_tree(token_list, Some(pass_arg));
                }
                else {
                    // Operator 2 should execute first
                    // Get the next argument
                    arg2 = recursive_generate_tree(token_list, Some(arg2))?;
                }
            }
            (_, _) => {

            }
        }
    }
}

// Consumes token list
pub fn generate_tree(token_list: &mut TokenList) -> CompilerResult<Option<OperatorElement>> {
    let mut arg1: Option<OperatorElement> = None;
    // See if there are still elements left
    while token_list.tokens.front().is_some() {
        // Fetch the next argument
        arg1 = Some(recursive_generate_tree(token_list, arg1)?);
    }
    return Ok(arg1);
}

struct ParseTree {

}

pub fn parse(token_list: TokenList) {
    
}