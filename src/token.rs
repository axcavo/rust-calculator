use std::fmt;


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OperatorType {
    Add,
    Subtract,
    Multiply,
    Divide
}

impl fmt::Display for OperatorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OperatorType::Add => write!(f, "Add"),
            OperatorType::Subtract => write!(f, "Subtract"),
            OperatorType::Multiply => write!(f, "Multiply"),
            OperatorType::Divide => write!(f, "Divide"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Operand,
    Operator(OperatorType),
    LeftParen,
    RightParen,
    Eof
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::Operand => write!(f, "Operand"),
            TokenType::Operator(op) => write!(f, "Operator({})", op),
            TokenType::LeftParen => write!(f, "LeftParen"),
            TokenType::RightParen => write!(f, "RightParen"),
            TokenType::Eof => write!(f, "Eof")
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub r#type: TokenType,
    pub lexeme: String,
    pub position: usize
}

impl Token {
    pub fn new(r#type: TokenType, lexeme: String, position: usize) -> Self {
        Self { r#type, lexeme, position }
    }

    pub fn operator_type(&self) -> Option<OperatorType> {
        match self.r#type {
            TokenType::Operator(op) => Some(op),
            _ => None,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "type: {}, lexeme: {}, position: {}", self.r#type, self.lexeme, self.position)
    }
}