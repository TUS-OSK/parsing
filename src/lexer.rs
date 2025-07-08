use crate::token;
use crate::token::{Token, TokenType};

#[cfg(test)]
pub mod lexer_test {

    use crate::token::TokenType;
    use crate::lexer::Lexer;

    #[test]
    pub fn test_next_token() {
        let input = "\
let five =5;
let ten = 10;
let add = fn(x, y) {
    x + y;
};
let result = add(five, ten);";
        
        struct TokenTest {
            expected_type: TokenType,
            expected_literal: String
        }
        
        impl TokenTest {
            fn new(expected_type: TokenType, expected_literal: impl Into<String>) -> TokenTest {
                TokenTest { expected_type, expected_literal: expected_literal.into() }
            }
        }
        
        let tests = vec![
            TokenTest::new(TokenType::Let, "let"),
            TokenTest::new(TokenType::Ident, "five"),
            TokenTest::new(TokenType::Assign, "="),
            TokenTest::new(TokenType::Int, "5"),
            TokenTest::new(TokenType::Semicolon, ";"),
            TokenTest::new(TokenType::Let, "let"),
            TokenTest::new(TokenType::Ident, "ten"),
            TokenTest::new(TokenType::Assign, "="),
            TokenTest::new(TokenType::Int, "10"),
            TokenTest::new(TokenType::Semicolon, ";"),
            TokenTest::new(TokenType::Let, "let"),
            TokenTest::new(TokenType::Ident, "add"),
            TokenTest::new(TokenType::Assign, "="),
            TokenTest::new(TokenType::Function, "fn"),
            TokenTest::new(TokenType::LParen, "("),
            TokenTest::new(TokenType::Ident, "x"),
            TokenTest::new(TokenType::Comma, ","),
            TokenTest::new(TokenType::Ident, "y"),
            TokenTest::new(TokenType::RParen, ")"),
            TokenTest::new(TokenType::LBrace, "{"),
            TokenTest::new(TokenType::Ident, "x"),
            TokenTest::new(TokenType::Plus, "+"),
            TokenTest::new(TokenType::Ident, "y"),
            TokenTest::new(TokenType::Semicolon, ";"),
            TokenTest::new(TokenType::RBrace, "}"),
            TokenTest::new(TokenType::Semicolon, ";"),
            TokenTest::new(TokenType::Let, "let"),
            TokenTest::new(TokenType::Ident, "result"),
            TokenTest::new(TokenType::Assign, "="),
            TokenTest::new(TokenType::Ident, "add"),
            TokenTest::new(TokenType::LParen, "("),
            TokenTest::new(TokenType::Ident, "five"),
            TokenTest::new(TokenType::Comma, ","),
            TokenTest::new(TokenType::Ident, "ten"),
            TokenTest::new(TokenType::RParen, ")"),
            TokenTest::new(TokenType::Semicolon, ";"),
            TokenTest::new(TokenType::EOF, "")
        ];
        
        let mut lexer = Lexer::new(input);
        
        for (i, tt) in tests.iter().enumerate() {
            let tok = lexer.next_token();
            assert_eq!(tok.token_type, tt.expected_type, "tests[{i}] - tokentype wrong. expected={expected}, got={got}",
                       expected = tt.expected_type.as_str(), got = tok.token_type.as_str());
            assert_eq!(tok.literal, tt.expected_literal, "tests[{i}] - literal wrong. expected={expected}, got={got}",
                       expected = tt.expected_literal, got = tok.literal);
        }
    }
}

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