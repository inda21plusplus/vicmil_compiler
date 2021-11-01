use std::{collections::LinkedList, panic};


#[derive(Debug)]
pub struct Token {
    pub text: String,
    pub start_col: usize,
    pub line: usize,
    pub token_type: TokenType
}
impl Token {
    pub fn new(text: String, start_col: usize, line: usize, token_type: TokenType) -> Self {
        Self {
            text,
            start_col,
            line,
            token_type
        }
    }
    pub fn to_string(&self) -> String {
        return "('".to_string() 
        + self.text.as_str() 
        + "', '" 
        + format!("{:?}", self.token_type).as_str() 
        + "', ln:"
        + self.line.to_string().as_str() 
        + ", col:"
        + self.start_col.to_string().as_str()
        + ")";
    }
}

pub struct TokenList {
    pub tokens: LinkedList<Token>
}

impl TokenList {
    pub fn new() -> Self {
        Self {
            tokens: LinkedList::new()
        }
    }
    pub fn to_string(&self) -> String {
        let mut text = String::from("{");
        let mut i = 0;

        for token in self.tokens.iter() {
            if i != 0 {
                text += ", "
            }
            text += token.to_string().as_str();
            i += 1;
        }

        text += "}";

        return text;
    }
}

pub struct TokenConstructor {
    text: String,
    token_type: Option<TokenType>,
    pub line: usize,
    pub col: usize,
    start_col: usize
}

impl TokenConstructor {
    pub fn new() -> Self {
        Self {
            text: "".to_string(),
            token_type: None,
            line: 0,
            col: 0,
            start_col: 0
        }
    }
    pub fn get_symbol_type(char_: char) -> Option<TokenType> {
        match char_ {
            '=' | '+' | '-' | '/' | '.' | '*' | '%' | '!' | '>' | '<' | ':' => {
                return Some(TokenType::Operator);
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                return Some(TokenType::Number);
            }
            _ => {
            }
        }
        return Some(TokenType::Identifier);
    }
    pub fn is_operator(&self) -> bool{
        match self.text.as_str() {
            "=" | "+" | "-" | "/" | "." | "*" | "%" | "!" | "+=" | "==" | "-=" | "!=" | "as"=> {
                return true;
            }
            _ => {
                return false;
            }
        }
    }
    pub fn is_identifier(&self) -> bool {
        if let Some(TokenType::Identifier) = self.token_type {
            return true;
        }
        return false;
    }
    pub fn is_number(&self) -> bool {
        if let Some(TokenType::Number) = self.token_type {
            return true;
        }
        return false;
    }
    pub fn process_text(&mut self, token_list: &mut TokenList) {
        match self.text.as_str() {
            "" => {}
            "(" => {
                let return_token =  Some(Token::new(self.text.clone(), self.start_col, self.line, TokenType::LParen));
                token_list.tokens.push_back(return_token.unwrap());
            }
            ")" => {
                let return_token =  Some(Token::new(self.text.clone(), self.start_col, self.line, TokenType::RParen));
                token_list.tokens.push_back(return_token.unwrap());
            }
            "{" => {
                let return_token =  Some(Token::new(self.text.clone(), self.start_col, self.line, TokenType::Other));
                token_list.tokens.push_back(return_token.unwrap());
            }
            "}" => {
                let return_token =  Some(Token::new(self.text.clone(), self.start_col, self.line, TokenType::Other));
                token_list.tokens.push_back(return_token.unwrap());
            }
            ";" => {
                let return_token =  Some(Token::new(self.text.clone(), self.start_col, self.line, TokenType::Semicolon));
                token_list.tokens.push_back(return_token.unwrap());
            }
            _ => {
                if self.is_operator() {
                    let return_token =  Some(Token::new(self.text.clone(), self.start_col, self.line, TokenType::Operator));
                    token_list.tokens.push_back(return_token.unwrap());
                }
                else if self.is_number() {
                    let return_token =  Some(Token::new(self.text.clone(), self.start_col, self.line, TokenType::Number));
                    token_list.tokens.push_back(return_token.unwrap());
                }
                else if self.is_identifier() {
                    let return_token =  Some(Token::new(self.text.clone(), self.start_col, self.line, TokenType::Identifier));
                    token_list.tokens.push_back(return_token.unwrap());
                }
                else {
                    panic!("Could not parse!");
                }
            }
        }
        self.start_col = self.col;
    }
    pub fn new_line(&mut self) {
        self.line += 1;
        self.col = 0;
        self.start_col = 0;
    }
    pub fn insert(&mut self, char_: char, token_list: &mut TokenList) {
        self.col += 1;
        match char_ {
            ' ' | '\t' => {
                self.process_text(token_list);
                self.text = "".to_string();
                self.token_type = None;
                self.start_col = self.col;
            }
            ')' | '(' | '}' | '{' | ';' => {
                self.process_text(token_list);
                self.text = char_.to_string();
                self.token_type = None;
                self.process_text(token_list);
                self.text = "".to_string();
                self.token_type = None;
            }
            '\n' => {
                self.process_text(token_list);
                self.text = "".to_string();
                self.token_type = None;
                self.new_line();
            }
            _ => {
                if let Some(TokenType::Operator) = self.token_type {
                    if let Some(TokenType::Operator) = Self::get_symbol_type(char_) {
                        self.text += char_.to_string().as_str();
                    }
                    else {
                        self.process_text(token_list);
                        self.text = char_.to_string();
                        self.token_type = Self::get_symbol_type(char_);
                    }
                }
                else if let Some(TokenType::Identifier) = self.token_type {
                    if let Some(TokenType::Operator) = Self::get_symbol_type(char_) {
                        self.process_text(token_list);
                        self.text = char_.to_string();
                        self.token_type = Self::get_symbol_type(char_);
                    }
                    else {
                        self.text += char_.to_string().as_str();
                    }
                }
                else if let Some(TokenType::Number) = self.token_type {
                    if let Some(TokenType::Operator) = Self::get_symbol_type(char_) {
                        self.process_text(token_list);
                        self.text = char_.to_string();
                        self.token_type = Self::get_symbol_type(char_);
                    }
                    else {
                        self.text += char_.to_string().as_str();
                    }
                }
                else {
                    self.text = char_.to_string();
                    self.token_type = Self::get_symbol_type(char_);
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum TokenType {
    Number,
    Identifier,
    Operator,
    LParen,
    RParen,
    Semicolon,
    Other
}

pub struct IterRef<'t> {
    char_num: usize,
    iter: core::slice::Iter<'t, u8>,
    len: usize
}

impl<'t> IterRef<'t> {
    pub fn new(text: &'t str) -> Self {
        Self {
            char_num: 0,
            len: text.len(),
            iter: text.as_bytes().iter()
        }
    }
    pub fn next(&mut self) -> Option<char> {
        if !self.reached_end() {
            self.char_num += 1;
            return Some(*self.iter.next().unwrap() as char);
        }
        return None;
    }
    pub fn reached_end(&self) -> bool {
        return self.len <= self.char_num;
    }
}

pub fn tokenize(text: &str) -> TokenList {
    let mut iter_ref = IterRef::new(text);
    let mut token_constructor = TokenConstructor::new();
    let mut token_list = TokenList::new();
    while !iter_ref.reached_end() {
        let char_ = iter_ref.next().unwrap();
        token_constructor.insert(char_, &mut token_list);
    }
    token_constructor.process_text(&mut token_list);
    return token_list;
}