use std::collections::LinkedList;


pub struct TokenExpressions {
    pub token_lists: LinkedList<TokenList>
}
impl TokenExpressions {
    pub fn new() -> Self {
        Self {
            token_lists: LinkedList::new()
        }
    }
    pub fn to_string(&self) -> String {
        let mut text = String::from("{  ");
        let mut i = 0;

        for token in self.token_lists.iter() {
            if i != 0 {
                text += ", "
            }
            text += token.to_string().as_str();
            i += 1;
        }

        text += "  }";

        return text;
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
    pub fn add_token(&mut self, token: Token) {
        if token.text.len() != 0 {
            self.tokens.push_back(token)
        }
    }
    pub fn add_token_from_text(&mut self, text: String, token_type: TokenType, line_num: u32, col_num: u32) {
        if text.len() != 0 {
            self.tokens.push_back(Token::new(text, token_type, line_num, col_num));
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

#[derive(Debug, Clone)]
pub struct Token {
    pub text: String,
    pub token_type: TokenType,
    pub line_num: u32,
    pub col_num: u32
}
impl Token {
    pub fn new(text: String, token_type: TokenType, line_num: u32, col_num: u32) -> Self {
        Self {
            text,
            token_type,
            line_num,
            col_num
        }
    }
    pub fn to_string(&self) -> String {
        return "('".to_string() 
        + self.text.as_str() 
        + "', '" 
        + format!("{:?}", self.token_type).as_str() 
        + "', ln:"
        + self.line_num.to_string().as_str() 
        + ", col:"
        + self.col_num.to_string().as_str()
        + ")";
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    Operator(u32), // In which order to execute, 0 first etc.
    IdentifierOrNumber,
    LParen,
    RParen,
    LCurrBracket,
    RCurrBracket,
    Identifier,
    Number,
    Semicolon,
}

pub fn tokenize(text: &str) -> TokenExpressions {
    let mut line_num: u32 = 0;
    let mut col_num: u32 = 0;
    let mut curr_text: String = String::from("");
    let mut token_expressions = TokenExpressions::new();
    let mut curr_token_list = TokenList::new();
    for i in text.as_bytes().into_iter() {
        col_num += 1;
        let i_char: char = *i as char;
        match i_char {
            ' ' => {
                curr_token_list.add_token_from_text(curr_text, TokenType::IdentifierOrNumber, line_num, col_num);
                curr_text = String::from("");
            }
            '\n' => {
                line_num += 1;
                col_num = 0;
                curr_token_list.add_token_from_text(curr_text, TokenType::IdentifierOrNumber, line_num, col_num);
                curr_text = String::from("");
            }
            '.' => {
                curr_token_list.add_token_from_text(curr_text, TokenType::IdentifierOrNumber, line_num, col_num);
                curr_text = String::from("");
                curr_token_list.add_token_from_text(i_char.to_string(), TokenType::Operator(0), line_num, col_num);
            }
            '/' | '*' => {
                curr_token_list.add_token_from_text(curr_text, TokenType::IdentifierOrNumber, line_num, col_num);
                curr_text = String::from("");
                curr_token_list.add_token_from_text(i_char.to_string(), TokenType::Operator(1), line_num, col_num);
            }
            '+' | '-' => {
                curr_token_list.add_token_from_text(curr_text, TokenType::IdentifierOrNumber, line_num, col_num);
                curr_text = String::from("");
                curr_token_list.add_token_from_text(i_char.to_string(), TokenType::Operator(2), line_num, col_num);
            }
            '!' | '|' | '&' | '>' | '<' => {
                curr_token_list.add_token_from_text(curr_text, TokenType::IdentifierOrNumber, line_num, col_num);
                curr_text = String::from("");
                curr_token_list.add_token_from_text(i_char.to_string(), TokenType::Operator(3), line_num, col_num);
            }
            ',' => {
                curr_token_list.add_token_from_text(curr_text, TokenType::IdentifierOrNumber, line_num, col_num);
                curr_text = String::from("");
                curr_token_list.add_token_from_text(i_char.to_string(), TokenType::Operator(4), line_num, col_num);
            }
            ':' => {
                curr_token_list.add_token_from_text(curr_text, TokenType::IdentifierOrNumber, line_num, col_num);
                curr_text = String::from("");
                curr_token_list.add_token_from_text(i_char.to_string(), TokenType::Operator(5), line_num, col_num);
            }
            '=' => {
                curr_token_list.add_token_from_text(curr_text, TokenType::IdentifierOrNumber, line_num, col_num);
                curr_text = String::from("");
                curr_token_list.add_token_from_text(i_char.to_string(), TokenType::Operator(6), line_num, col_num);
            }
            ';' => {
                curr_token_list.add_token_from_text(curr_text, TokenType::IdentifierOrNumber, line_num, col_num);
                curr_text = String::from("");
                //curr_token_list.add_token_from_text(i_char.to_string(), TokenType::Semicolon, line_num, col_num);

                // Add current token list as an expression
                if curr_token_list.tokens.len() != 0 {
                    token_expressions.token_lists.push_back(curr_token_list);
                    curr_token_list = TokenList::new();
                }
            }
            '(' => {
                curr_token_list.add_token_from_text(curr_text, TokenType::IdentifierOrNumber, line_num, col_num);
                curr_text = String::from("");
                curr_token_list.add_token_from_text(i_char.to_string(), TokenType::LParen, line_num, col_num);
            }
            ')' => {
                curr_token_list.add_token_from_text(curr_text, TokenType::IdentifierOrNumber, line_num, col_num);
                curr_text = String::from("");
                curr_token_list.add_token_from_text(i_char.to_string(), TokenType::RParen, line_num, col_num);
            }
            '{' => {
                curr_token_list.add_token_from_text(curr_text, TokenType::IdentifierOrNumber, line_num, col_num);
                curr_text = String::from("");
                curr_token_list.add_token_from_text(i_char.to_string(), TokenType::LCurrBracket, line_num, col_num);
            }
            '}' => {
                curr_token_list.add_token_from_text(curr_text, TokenType::IdentifierOrNumber, line_num, col_num);
                curr_text = String::from("");
                curr_token_list.add_token_from_text(i_char.to_string(), TokenType::RCurrBracket, line_num, col_num);
            }
            _ => {
                curr_text.push(i_char);
            }
        }
    }
    // Add remaining text as a token
    curr_token_list.add_token_from_text(curr_text, TokenType::IdentifierOrNumber, line_num, col_num);

    // Add current token list as an expression
    if curr_token_list.tokens.len() != 0 {
        token_expressions.token_lists.push_back(curr_token_list);
        curr_token_list = TokenList::new();
    }
    return token_expressions;
}