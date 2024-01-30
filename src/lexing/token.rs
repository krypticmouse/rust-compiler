#[derive(Debug)]
pub enum Token{
    // Keywords
    Print(String),
    If(String),
    Else(String),
    Int(String),

    // Literals
    IntegerLiteral(i32),
    StringLiteral(String),

    // Identifiers
    Identifier(String),

    // Operators
    Plus(String),
    Assign(String),

    // Punctuation
    Semicolon(String),
    LeftParen(String),
    RightParen(String),
    LeftBrace(String),
    RightBrace(String),

    // Logical Operators
    GreaterThan(String),
    LessThan(String),
}

impl Token {
    pub fn get_token(token_type: &str, value: Option<&str>) -> Token {
        match token_type {
            "Print" => Token::Print("print".to_string()),
            "If" => Token::If("if".to_string()),
            "Else" => Token::Else("else".to_string()),
            "Int" => Token::Int("int".to_string()),
            "IntegerLiteral" => Token::IntegerLiteral(value.unwrap().parse::<i32>().unwrap()),
            "StringLiteral" => Token::StringLiteral(value.unwrap().to_string()),
            "Identifier" => Token::Identifier(value.unwrap().to_string()),
            "Plus" => Token::Plus("+".to_string()),
            "Assign" => Token::Assign("=".to_string()),
            "Semicolon" => Token::Semicolon(";".to_string()),
            "LeftParen" => Token::LeftParen("(".to_string()),
            "RightParen" => Token::RightParen(")".to_string()),
            "LeftBrace" => Token::LeftBrace("{".to_string()),
            "RightBrace" => Token::RightBrace("}".to_string()),
            "GreaterThan" => Token::GreaterThan(">".to_string()),
            "LessThan" => Token::LessThan("<".to_string()),
            _ => panic!("Invalid token type: {}", token_type),
        }
    }

    pub fn get_token_regex(token_type: &str) -> String {
        match token_type {
            "Print" => r"print",
            "If" => r"if",
            "Else" => r"else",
            "Int" => r"int\s+",
            "IntegerLiteral" => r"\d+",
            "StringLiteral" => r#"\".*\""#,
            "Identifier" => r"[a-zA-Z_][a-zA-Z0-9_]* =",
            "Plus" => r"\+",
            "Assign" => r"=",
            "Semicolon" => r";",
            "LeftParen" => r"\(",
            "RightParen" => r"\)",
            "LeftBrace" => r"\{",
            "RightBrace" => r"}",
            "GreaterThan" => r">",
            "LessThan" => r"<",
            _ => panic!("Invalid token type: {}", token_type),
        }.to_string()
    }
}