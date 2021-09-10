use crate::main;
use std::num::NonZeroUsize;
use crate::tokens;
use crate::tokens::TokenType::{BANG_EQUAL, EQUAL_EQUAL, LESS_EQUAL, GREATER_EQUAL, GREATER};

struct Scanner { 
    source: &str,
    tokens: Vec<Token>,
    start: u32, 
    current: u32,
    line: NonZeroUsize,
}


impl Scanner { 

    pub const fn new(source: &str) -> Self {

        Self { 
            source, 
            tokens: Vec::new(), 
            start: 0, 
            current: 0, 
            line: NonZeroUsize::new(1).unwrap(), 
        }
    }

    fn scan_tokens(&mut self) -> Vec<Token>{ 
        while !self.is_at_end() {
            //beginning of next lexeme 
            let start = self.current;
            self.scan_token(); 
        }

        self.tokens.push(Token(EOF, "", null, line));
        self.tokens.clone()
    }

    fn scan_token(&mut self) { 
        let c: char = self.advance(); 
        match c  { 
            '(' => add_token(LEFT_PAREN),
            ')' => add_token(RIGHT_PAREN),
            '{' => add_token(LEFT_BRACE), 
            '}' => add_token(RIGHT_BRACE),
            ',' => add_token(COMMA), 
            '.' => add_token(DOT), 
            '-' => add_token(MINUS), 
            '+' => add_token(PLUS), 
            ';' => add_token(SEMICOLON), 
            '*' => add_token(STAR),
            '!' => add_token(if check_forward("=") {BANG_EQUAL} else {BANG}),
            '=' => add_token(if check_forward('=') {EQUAL_EQUAL} else {EQUAL}),
            '<' => add_token(if check_forward('=') {LESS_EQUAL} else {LESS}),
            '>' => add_token(if check_forward('=') {GREATER_EQUAL} else {GREATER}),
            _ => error(self.line, "Unexpected Character")
        }
    }

    fn check_forward(&mut self,expected: char) -> bool {
        self.is_at_end() || self.source[current] != expected

    }

    fn is_at_end(&mut self) -> bool { 
        self.current >= self.source.len() as u32
    }

    fn advance(&mut self) -> char {
        self.source[Scanner.current + 1]

    }

    fn add_token(&mut self ,type_: TokenType) {
        let text: String = String::from(&self.source[self.start..self.source]);
        &self.tokens.push(
            Token::new(type_, text, literal, self.line)
        );


    }
}