use crate::errors::ScanError;
use crate::token::OperatorType;
use crate::token::Token;
use crate::token::TokenType;


pub fn scan(source: String) -> Result<Vec<Token>, ScanError> {
    let mut tokens = Vec::new();
    let mut current: usize = 0;

    while !is_at_end(&source, &current) {
        skip_whitespace(&source, &mut current);

        if is_at_end(&source, &current) {
            break;
        }

        let token = scan_token(&source, &mut current)?;
        tokens.push(token);
    }

    tokens.push(Token::new(TokenType::Eof, String::new(), current));
    Ok(tokens)
}

fn scan_token(source: &String, current: &mut usize) -> Result<Token, ScanError> {
    let mut start = *current;
    let c = advance(source, current).unwrap();

    match c {
        '(' => Ok(create_token(TokenType::LeftParen, c.to_string(), *current)),
        ')' => Ok(create_token(TokenType::RightParen, c.to_string(), *current)),
        '+' => Ok(create_token(TokenType::Operator(OperatorType::Add), c.to_string(), *current)),
        '-' => Ok(create_token(TokenType::Operator(OperatorType::Subtract), c.to_string(), *current)),
        '*' => Ok(create_token(TokenType::Operator(OperatorType::Multiply), c.to_string(), *current)),
        '/' => Ok(create_token(TokenType::Operator(OperatorType::Divide), c.to_string(), *current)),
        _ => {
            if c.is_ascii_digit() {
                return Ok(create_number_token(source, current, &mut start));
            }
            Err(ScanError::UnexpectedChar(c, start))
        }
    }
}

fn create_token(token_type: TokenType, lexeme: String, position: usize) -> Token {
    Token::new(token_type, lexeme, position)
}

fn peek(source: &String, current: &usize) -> char {
    source.chars().nth(*current).unwrap_or('\0')
}

fn peek_next(source: &String, current: &usize) -> char {
    source.chars().nth(*current + 1).unwrap_or('\0')
}


fn skip_whitespace(source: &String, current: &mut usize) {
    while let Some(c) = source.chars().nth(*current) {
        if c.is_whitespace() {
            *current += 1;
        } else {
            break;
        }
    }
}

fn create_number_token(source: &String, current: &mut usize, start: &mut usize) -> Token {
    while peek(source, current).is_digit(10u32) {
        advance(source, current);
    }

    if peek(source, current) == '.' && peek_next(source, current).is_digit(10u32) {
        advance(source, current);

        while peek(source, current).is_digit(10u32) {
            advance(source, current);
        }
    }

    let lexeme: String = source.chars().skip(*start).take(*current - *start).collect();
    let lexeme = lexeme.trim().to_string();

    Token::new(TokenType::Operand, lexeme, *start)
}

fn advance(source: &String, current: &mut usize) -> Option<char> {
    let c = source.chars().nth(*current)?;
    *current += 1;
    Some(c)
}

fn is_at_end(source: &String, current: &usize) -> bool {
    *current >= source.len()
}
