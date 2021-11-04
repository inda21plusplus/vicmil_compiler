/*pub mod compiler_error {
    use trait_enum::*;
    use crate::lexer::*;
    #[allow(dead_code)]
    pub type CompilerResult<T> = std::result::Result<T, CompilerError>;

    pub trait CompilerErrorCommonTrait {
        fn compiler_err_to_string(&self) -> String;
    }

    trait_enum!{
        #[derive(Debug)]
        pub enum CompilerError: CompilerErrorCommonTrait {
            String,
            TokenError
        }
    }

    #[derive(Debug)]
    pub enum TokenErrorEnum {
        Default,
        UnexpectedOperator,
        ExpectedOperator,
        UnexpectedType,
        InvalidIdentifierOperation
    }

    #[derive(Debug)]
    pub struct TokenError {
        pub token: Token,
        pub token_error: TokenErrorEnum
    }
    impl TokenError {
        pub fn new(token: Token, token_error: TokenErrorEnum) -> Self {
            Self {
                token,
                token_error
            }
        }
    }
    impl From<TokenError> for CompilerError {
        fn from(err: TokenError) -> Self {
            return CompilerError::TokenError(err);
        }
    }

    impl CompilerErrorCommonTrait for TokenError {
        fn compiler_err_to_string(&self) -> String {
            return format!("{:?}", self.token_error).to_string()
            + ": " 
            + self.token.to_string().as_str();
        }
    }

    impl std::fmt::Display for CompilerError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({})", self.compiler_err_to_string())
        }
    }

    impl CompilerErrorCommonTrait for String {
        fn compiler_err_to_string(&self) -> String {
            return self.clone();
        }
    }
    impl std::convert::From<String> for CompilerError {
        fn from(err: String) -> Self {
            return CompilerError::String(err);
        }
    }
    impl std::convert::From<&str> for CompilerError {
        fn from(err: &str) -> Self {
            return CompilerError::String(err.to_string());
        }
    }
}
*/