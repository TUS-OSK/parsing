#[cfg(test)]
pub mod lexer_test {

    use crate::token::TokenType;
    use crate::lexer::Lexer;

    #[test]
    pub fn test_next_token() {
        let input = "\
let five = 5;
let ten = 10;
let add = fn(x, y) {
    x + y;
};
let result = add(five, ten);
!-/*5;
5 < 10 > 5;
";

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