#[cfg(test)]

mod compiler_test {
    use crate::tokenizer::*;
    use crate::parser::*;
    use crate::error_handler::*;

    #[test]
    fn tests_working() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn tokenizer_a_plus_b() {
        let data = "a + b".to_string();
        let mut tokanized_text = tokenize(&data);
        let mut token_list = tokanized_text.token_lists.pop_front().unwrap();
        assert_eq!(tokanized_text.token_lists.front().is_none(), true);
        match token_list.tokens.pop_front().unwrap() {
            Token{token_type: TokenType::IdentifierOrNumber, ..} => {}
            Token{token_type: TokenType::Identifier, ..} => {}
            _ => {
                panic!("Invalid type")
            }
        }
        match token_list.tokens.pop_front().unwrap() {
            Token{token_type: TokenType::Operator(_), ..} => {}
            _ => {
                panic!("Invalid type")
            }
        }
        match token_list.tokens.pop_front().unwrap() {
            Token{token_type: TokenType::IdentifierOrNumber, ..} => {}
            Token{token_type: TokenType::Identifier, ..} => {}
            _ => {
                panic!("Invalid type")
            }
        }
        assert_eq!(token_list.tokens.front().is_none(), true);
    }

    #[test]
    fn tokenizer_abc_plus_xyz() {
        let data = "abc + xyz".to_string();
        let mut tokanized_text = tokenize(&data);
        let mut token_list = tokanized_text.token_lists.pop_front().unwrap();
        assert_eq!(tokanized_text.token_lists.front().is_none(), true);
        match token_list.tokens.pop_front().unwrap() {
            Token{token_type: TokenType::IdentifierOrNumber, text ,..} => {
                assert_eq!(text, "abc".to_string());
            }
            Token{token_type: TokenType::Identifier, text ,..} => {
                assert_eq!(text, "abc".to_string());
            }
            _ => {
                panic!("Invalid type")
            }
        }
        match token_list.tokens.pop_front().unwrap() {
            Token{token_type: TokenType::Operator(_), text ,..} => {
                assert_eq!(text, "+".to_string());
            }
            _ => {
                panic!("Invalid type")
            }
        }
        match token_list.tokens.pop_front().unwrap() {
            Token{token_type: TokenType::IdentifierOrNumber, text ,..} => {
                assert_eq!(text, "xyz".to_string());
            }
            Token{token_type: TokenType::Identifier, text ,..} => {
                assert_eq!(text, "xyz".to_string());
            }
            _ => {
                panic!("Invalid type")
            }
        }
        assert_eq!(token_list.tokens.front().is_none(), true);
    }

    #[test]
    fn tokenizer_a_minus_bc_plus_xyz() {
        let data = "a - bc + xyz".to_string();
        let mut tokanized_text = tokenize(&data);
        let mut token_list = tokanized_text.token_lists.pop_front().unwrap();
        assert_eq!(tokanized_text.token_lists.front().is_none(), true);
        match token_list.tokens.pop_front().unwrap() {
            Token{token_type: TokenType::IdentifierOrNumber, text ,..} => {
                assert_eq!(text, "a".to_string());
            }
            Token{token_type: TokenType::Identifier, text ,..} => {
                assert_eq!(text, "a".to_string());
            }
            _ => {
                panic!("Invalid type")
            }
        }
        match token_list.tokens.pop_front().unwrap() {
            Token{token_type: TokenType::Operator(_), text ,..} => {
                assert_eq!(text, "-".to_string());
            }
            _ => {
                panic!("Invalid type")
            }
        }
        match token_list.tokens.pop_front().unwrap() {
            Token{token_type: TokenType::IdentifierOrNumber, text ,..} => {
                assert_eq!(text, "bc".to_string());
            }
            Token{token_type: TokenType::Identifier, text ,..} => {
                assert_eq!(text, "bc".to_string());
            }
            _ => {
                panic!("Invalid type")
            }
        }
        match token_list.tokens.pop_front().unwrap() {
            Token{token_type: TokenType::Operator(_), text ,..} => {
                assert_eq!(text, "+".to_string());
            }
            _ => {
                panic!("Invalid type")
            }
        }
        match token_list.tokens.pop_front().unwrap() {
            Token{token_type: TokenType::IdentifierOrNumber, text ,..} => {
                assert_eq!(text, "xyz".to_string());
            }
            Token{token_type: TokenType::Identifier, text ,..} => {
                assert_eq!(text, "xyz".to_string());
            }
            _ => {
                panic!("Invalid type")
            }
        }
        assert_eq!(token_list.tokens.front().is_none(), true);
    }

    

    #[test]
    fn parser_test() {
        
    }
}