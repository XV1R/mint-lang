use std::num::NonZeroUsize;

pub enum TokenType {
    //Single-character tokens 
    LEFT_PAREN, RIGHT_PAREN,
    LEFT_BRACE, RIGHT_BRACE, 
    COMMA, DOT, MINUS, SEMICOLON, SLASH, STAR, 

    //One or two character tokens
    BANG, BANG_EQUAL,
    EQUAL, EQUAL_EQUAL, 
    GREATER, GREATER_EQUAL, 
    LESS, LESS_EQUAL, 

    //Literals
    IDENTIFIER, STRING, NUMBER,

    //keywords
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL,
    OR, PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

    EOF
}



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Token { 
    pub TokenType: TokenType,
    pub lexeme: String,
    pub literal: Option<literal>,
    pub line: NonZeroUsize,

}

impl Token {
    pub(crate) fn new(type_: TokenType, lexeme: String, literal: Option<literal>, line: NonZeroUsize) -> Self {
        Self { TokenType: type_, lexeme: lexeme, literal, line}
    }
}