mod lexer;
fn TestNextToken(t ) {
input := `=+(){},;`
tests := []struct {
expectedType
token.TokenType
expectedLiteral string
}{
{token.ASSIGN, "="},
{token.PLUS, "+"},
{token.LPAREN, "("},
{token.RPAREN, ")"},