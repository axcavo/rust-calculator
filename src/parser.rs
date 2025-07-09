use crate::{ast::Expr, token::{OperatorType, Token, TokenType}};


pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        self.expression()
    }

    fn expression(&mut self) -> Option<Expr> {
        let mut expr = self.term()?;

        while self.match_operator(&[OperatorType::Add, OperatorType::Subtract]) {
            let operator = self.previous().operator_type().unwrap();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Some(expr)
    }

    fn term(&mut self) -> Option<Expr> {
        let mut expr = self.factor()?;

        while self.match_operator(&[OperatorType::Multiply, OperatorType::Divide]) {
            let operator = self.previous().operator_type().unwrap();
            let right = self.factor()?;
            expr = Expr::Binary { 
                left: Box::new(expr), 
                operator, 
                right: Box::new(right), 
            };
        }

        Some(expr)
    }

    fn factor(&mut self) -> Option<Expr> {
        let token = self.peek();

        match &token.r#type {
            TokenType::Operand => {
                let value = token.lexeme.parse::<f64>().ok()?;
                self.advance(); 
                Some(Expr::Literal(value))
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression()?;
                if self.peek().r#type != TokenType::RightParen {
                    return None;
                }
                self.advance();
                Some(Expr::Grouping(Box::new(expr)))
            }
            _ => None, 
        }
    }


    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }

    fn match_operator(&mut self, types: &[OperatorType]) -> bool {
        if let TokenType::Operator(op) = &self.peek().r#type {
            if types.contains(op) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current+=1;
        }
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        return self.peek().r#type == TokenType::Eof;
    }

}