use std::{collections::LinkedList, default};

use crate::TokenList;

enum ScopeType {
    Scope,
    Parenthesis
}

pub struct Attribute {
    pub type_: String,
}

struct ScopeDictionary {
    pub scope_type: LinkedList<ScopeType>,
    attributes: LinkedList<std::collections::HashMap<String, Attribute>>,
}

impl ScopeDictionary {
    pub fn new() -> Self {
        Self {
            scope_type: LinkedList::new(),
            attributes: LinkedList::new()
        }
    }
    pub fn new_scope(&mut self, scope_type: ScopeType) {
        self.scope_type.push_front(scope_type);
        self.attributes.push_front(std::collections::HashMap::new());
    }
    pub fn undo_scope(&mut self) {
        self.scope_type.pop_front();
        self.attributes.pop_front();
    }
    pub fn scope_type(&mut self) -> &mut ScopeType {
        return self.scope_type.front_mut().unwrap();
    }
    pub fn insert(&mut self, key: String, attr: Attribute) {
        self.attributes.front_mut().unwrap().insert(key, attr);
    }
    pub fn get(&mut self, key: &String) -> Option<&mut Attribute> {
        for i in self.attributes.iter_mut() {
            if i.contains_key(key) {
                return i.get_mut(key);
            }
        }
        return None;
    }
}

fn parse_let_pattern() {

}

fn parse_fn_pattern() {
    
}

fn parse_struct_pattern() {

}

fn parse_for_pattern() {

}

fn parse_expression_pattern() {

}

pub fn parse(tokens: &mut TokenList) {
    let mut dictionary = ScopeDictionary::new();
    dictionary.new_scope(ScopeType::Scope);
    dictionary.insert("some text".to_string(), Attribute { type_: "cool type".to_string() });
    let attr = dictionary.get(&"some text".to_string());
    println!("{}", attr.unwrap().type_);
    dictionary.new_scope(ScopeType::Scope);
    let attr = dictionary.get(&"some text".to_string());
    println!("{}", attr.unwrap().type_);
    dictionary.insert("some other".to_string(), Attribute { type_: "cool other".to_string() });
    let attr = dictionary.get(&"some other".to_string());
    println!("{}", attr.unwrap().type_);
    dictionary.undo_scope();
    let attr = dictionary.get(&"some other".to_string());
    println!("{}", attr.unwrap().type_);

}

/*use crate::tokenizer::*;
use crate::error_handler::compiler_error::*;
use std::borrow::Borrow;
use std::collections::LinkedList;
use std::collections::linked_list::Iter as LinkedListIter;

pub enum OperatorElement {
    ExpressionTree(Box<ExpressionTree>),
    Token(Token),
    Parenthesis(Box<Parenthesis>),
    IdentifierOperation(Box<IdentifierOperation>)
}

pub struct Parenthesis {
    pub arg: OperatorElement,
}
impl Parenthesis {
    pub fn to_string(&self) -> String {
        let mut text = String::from("(");
        text += self.arg.to_string().as_str();
        /*let mut iter_num = 0;
        for i in self.arg_list.iter() {
            if iter_num != 0 {
                text += ", ";
            }
            text += i.to_string().as_str();
            iter_num += 1;
        }*/
        text += ")";
        return text;
    }
}

pub struct IdentifierOperation {
    pub operator: OperatorElement,
    pub arg2: OperatorElement,
}
impl IdentifierOperation {
    pub fn to_string(&self) -> String {
        return "(".to_string()
        + self.operator.to_string().as_str()
        + " "
        + self.arg2.to_string().as_str()
        + ")";
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
            OperatorElement::Parenthesis(token) => {
                return token.to_string();
            }
            OperatorElement::IdentifierOperation ( token ) => {
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

pub fn recursive_generate_tree_parenthesis(token_list: &mut TokenList, arg1_in: Option<OperatorElement>) -> CompilerResult<OperatorElement> {
    return Ok(OperatorElement::Parenthesis(Box::new(Parenthesis{arg: recursive_generate_tree(token_list, arg1_in)?})));
}

pub fn recursive_generate_tree(token_list: &mut TokenList, arg1_in: Option<OperatorElement>) -> CompilerResult<OperatorElement> {
    // To add: Function call handling
    if arg1_in.is_some() {
        println!("{}   :   {}\n\n", arg1_in.as_ref().unwrap().to_string(), token_list.to_string());
    }
    else {
        println!("None   :   {}\n\n", token_list.to_string());
    }

    let arg1: OperatorElement;
    if arg1_in.is_none() {
        // Find first argument
        let element_1 = token_list.tokens.pop_front();
        let element_1 = element_1.unwrap();

        // Parse argument
        if let TokenType::LParen = element_1.token_type {
            // If element_1 is parenthesis, parse that first
            //arg1 = recursive_generate_tree(token_list, None)?;
            arg1 = recursive_generate_tree_parenthesis(token_list, None)?;
        }
        else if let TokenType::RParen = element_1.token_type {
            // If element_1 is parenthesis, parse that first
            //arg1 = recursive_generate_tree(token_list, None)?;
            return Ok(OperatorElement::Token(Token::new("".to_string(), TokenType::RParen, element_1.line_num, element_1.col_num)));
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
    if operator1.is_none() {
        return Ok(arg1);
    }
    let operator1 = operator1.unwrap();
    if let TokenType::Operator(_) = operator1.token_type {
    }
    else if let TokenType::LParen = operator1.token_type {
        let paren_expression = recursive_generate_tree_parenthesis(token_list, None)?;
        return Ok(OperatorElement::ExpressionTree( Box::new(ExpressionTree{
            arg1,
            arg2: paren_expression,
            operator: operator1.clone()
        })));
    }
    else if let TokenType::RParen = operator1.token_type {
        return Ok(arg1);
    }
    // Check if it is an Identifier operation
    else if let TokenType::IdentifierOrNumber = operator1.token_type {
        // Check if arg1 is an identifier or Identifier operation
        match &arg1 {
            OperatorElement::Token(arg1_token) => {
                if let TokenType::IdentifierOrNumber = arg1_token.token_type {
                }
                else {
                    return Err(CompilerError::from(TokenError::new(operator1.clone(), TokenErrorEnum::ExpectedOperator)))
                }
            }
            OperatorElement::IdentifierOperation(token) => {
            }
            _ => {
                return Err(CompilerError::from(TokenError::new(operator1.clone(), TokenErrorEnum::ExpectedOperator)))
            }
        }
        let pass_arg = Box::new(IdentifierOperation{operator: arg1, arg2:  OperatorElement::Token(operator1)});
        return recursive_generate_tree(token_list, Some(OperatorElement::IdentifierOperation(pass_arg)));
    }
    else {
        // Check if it is an Identifier operation
        return Err(CompilerError::from(TokenError::new(operator1.clone(), TokenErrorEnum::ExpectedOperator)))
    }

    // Find second argument
    let element_2 = token_list.tokens.pop_front();
    let element_2 = element_2.unwrap();

    // Parse second argument
    let mut arg2: OperatorElement;
    if let TokenType::LParen = element_2.token_type {
        // If element_2 is parenthesis, parse that first
        arg2 = recursive_generate_tree_parenthesis(token_list, None)?;
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
        let operator2 = operator2.unwrap().clone();

        if let TokenType::Operator(_) = operator2.token_type {

        }
        else if let TokenType::LParen = operator2.token_type {
            match operator1.token_type {
                TokenType::Operator(op1) => {
                    if op1 == 0 {
                        // it is a dot, and that should execute first
                        let pass_arg = OperatorElement::ExpressionTree( Box::new(ExpressionTree{
                            arg1,
                            arg2,
                            operator: operator1.clone()
                        }));
                        return recursive_generate_tree(token_list, Some(pass_arg));
                    }
                    else {
                        arg2 = recursive_generate_tree(token_list, Some(arg2))?;
                        continue;
                    }
                }
                _ => {
                    panic!("Something went wrong");
                }
            }
        }
        else if let TokenType::RParen = operator2.token_type {
            return Ok(OperatorElement::ExpressionTree( Box::new(ExpressionTree{
                arg1,
                arg2,
                operator: operator1.clone()
            })));
        }
        else if let TokenType::IdentifierOrNumber = operator2.token_type {
            return Err(CompilerError::from(TokenError::new(operator2.clone(), TokenErrorEnum::InvalidIdentifierOperation)));
            //arg2 = recursive_generate_tree(token_list, Some(arg2))?;
            //continue;
        }
        else {
            return Err(CompilerError::from(TokenError::new(operator2.clone(), TokenErrorEnum::ExpectedOperator)))
        }

        // See if operator 2 should execute before operator 1
        match (operator1.token_type, operator2.token_type) {
            (TokenType::Operator(op1), TokenType::Operator(op2)) => {
                if op1 < op2 {
                    // Operator 1 should execute first
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

*/