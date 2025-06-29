#[cfg(test)]
pub mod lexer_test {

    use crate::token::TokenType;
    use crate::lexer::Lexer;

    #[test]
    fn test_next_token() {
        let input = String::from("=+(){},;");
        
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
            TokenTest::new(TokenType::ASSIGN, "="),
            TokenTest::new(TokenType::PLUS, "+"),
            TokenTest::new(TokenType::LPAREN, "("),
            TokenTest::new(TokenType::RPAREN, ")"),
            TokenTest::new(TokenType::LBRACE, "{"),
            TokenTest::new(TokenType::RBRACE, "}"),
            TokenTest::new(TokenType::COMMA, ","),
            TokenTest::new(TokenType::SEMICOLON, ";"),
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
    input: String,
    position: usize,
    read_position: usize,
    ch: u8
}

impl Lexer {

    pub fn new(input: String) -> Lexer {
        Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0
        }
    }

    pub fn read_char(&mut self) -> u8 {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
        self.ch
    }
}