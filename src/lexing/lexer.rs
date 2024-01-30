use regex::Regex;
use crate::lexing::token::Token;

pub fn lex_program(program: &str) -> Vec<Token> {
    let current_input = program;
    let tokens = [
        "Print",
        "If",
        "Else",
        "Int",
        "IntegerLiteral",
        "StringLiteral",
        "Identifier",
        "Plus",
        "Assign",
        "Semicolon",
        "LeftParen",
        "RightParen",
        "LeftBrace",
        "RightBrace",
        "GreaterThan",
        "LessThan",
    ];
    let mut match_vec: Vec<(&str, usize, usize)> = Vec::new();
    
    for token in tokens.iter() {
        let token_regex = Token::get_token_regex(token);
        let re = Regex::new(token_regex.as_str()).unwrap();

        let matched = re.find_iter(current_input);
        let all_matches = matched.collect::<Vec<_>>();
        
        if all_matches.len() == 0 {
            continue;
        }

        for m in all_matches.iter() {
            match_vec.push((token, m.start(), m.end()));
        }
    }
    match_vec.sort_by(|a, b| a.1.cmp(&b.1));

    let mut token_vec: Vec<Token> = Vec::new();
    for m in match_vec.iter() {
        token_vec.push(Token::get_token(m.0, Some(&current_input[m.1..m.2])));
    }

    token_vec
}