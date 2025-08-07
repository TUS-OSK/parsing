pub mod token {
    type Tokentype &str;
    type Token struct{
        Type:Tokentype,
        Literal:&str
    }
    const ILLEGAL = "ILLEGAL";
    const EOF = "EOF";
    // 識別子 + リテラル
    const IDENT = "IDENT"; // add, foobar, x, y, ...
    const INT = "INT"; // 1343456
    // 演算子
    const ASSIGN = "=";
    const PLUS = "+";
    // デリミタ
    const COMMA = ",";
    const SEMICOLON = ";";
    const LPAREN = "(";
    const RPAREN = ")";
    const LBRACE = "{";
    const RBRACE = "}";
    // キーワード
    const FUNCTION = "FUNCTION";
    const LET = "LET";
}
