#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    // 識別子 + リテラル
    IDENT, // 例: add, foobar, x, y, ...
    INT, // 1343456

    // 演算子
    ASSIGN,
    PLUS,

    // デリミタ
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // キーワード
    FUNCTION,
    LET
}

impl TokenType {

    pub fn as_str(&self) -> &'static str {
        match self {
            TokenType::ILLEGAL => "ILLEGAL",
            TokenType::EOF => "EOF",
            TokenType::IDENT => "IDENT",
            TokenType::INT => "INT",
            TokenType::ASSIGN => "=",
            TokenType::PLUS => "+",
            TokenType::COMMA => ",",
            TokenType::SEMICOLON => ";",
            TokenType::LPAREN => "(",
            TokenType::RPAREN => ")",
            TokenType::LBRACE => "{",
            TokenType::RBRACE => "}",
            TokenType::FUNCTION => "FUNCTION",
            TokenType::LET => "LET"
        }
    }
}