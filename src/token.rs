use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    Illegal,
    EOF,

    // 識別子 + リテラル
    Ident, // 例: add, foobar, x, y, ...
    Int, // 1343456

    // 演算子
    Assign,
    Plus,

    // デリミタ
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // キーワード
    Function,
    Let
}

impl TokenType {

    pub fn as_str(&self) -> &'static str {
        match self {
            TokenType::Illegal => "ILLEGAL",
            TokenType::EOF => "EOF",
            TokenType::Ident => "IDENT",
            TokenType::Int => "INT",
            TokenType::Assign => "=",
            TokenType::Plus => "+",
            TokenType::Comma => ",",
            TokenType::Semicolon => ";",
            TokenType::LParen => "(",
            TokenType::RParen => ")",
            TokenType::LBrace => "{",
            TokenType::RBrace => "}",
            TokenType::Function => "FUNCTION",
            TokenType::Let => "LET"
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String
}

impl Token {
    pub fn new(token_type: TokenType, literal: impl Into<String>) -> Token {
        Token {
            token_type,
            literal: literal.into()
        }
    }
}

static KEYWORDS: LazyLock<HashMap<&'static str, TokenType>> = LazyLock::new(|| {
    HashMap::from([
        ("fn", TokenType::Function),
        ("let", TokenType::Let)
    ])
});

pub fn lookup_ident(ident: impl AsRef<str>) -> TokenType {
    KEYWORDS.get(ident.as_ref()).copied().unwrap_or(TokenType::Ident)
}