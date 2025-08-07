mod lexer_test;

use crate::token;
use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char
}

impl Lexer {

    pub fn new(input: impl Into<String>) -> Lexer {
        let mut lexer = Lexer {
            input: input.into().chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0'
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) -> char {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
        self.ch
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.ch {
            '=' => Token::new(TokenType::Assign, self.ch),
            '+' => Token::new(TokenType::Plus, self.ch),
            ',' => Token::new(TokenType::Comma, self.ch),
            ';' => Token::new(TokenType::Semicolon, self.ch),
            '(' => Token::new(TokenType::LParen, self.ch),
            ')' => Token::new(TokenType::RParen, self.ch),
            '{' => Token::new(TokenType::LBrace, self.ch),
            '}' => Token::new(TokenType::RBrace, self.ch),
            '\0' => Token::new(TokenType::EOF, ""),
            _ => {
                if Lexer::is_letter(self.ch) {
                    let literal = self.read_identifier();
                    return Token::new(token::lookup_ident(&literal), literal)
                } else if Lexer::is_digit(self.ch) {
                    return Token::new(TokenType::Int, self.read_number())
                } else {
                    Token::new(TokenType::Illegal, self.ch)
                }
            }
        };
        self.read_char();
        token
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        while Lexer::is_letter(self.ch) {
            ident.push(self.ch);
            self.read_char();
        }
        ident
    }

    fn is_letter(ch: char) -> bool {
        'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
    }

    fn read_number(&mut self) -> String {
        let mut number = String::new();
        while Lexer::is_digit(self.ch) {
            number.push(self.ch);
            self.read_char();
        }
        number
    }

    fn is_digit(ch: char) -> bool {
        '0' <= ch && ch <= '9'
    }
}