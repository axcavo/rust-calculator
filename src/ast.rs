use crate::token::OperatorType;

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: OperatorType,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Literal(f64),
}

pub fn evaluate(expr: &Expr) -> f64 {
    match expr {
        Expr::Literal(value) => *value,
        Expr:: Grouping(inner) => evaluate(inner),
        Expr::Binary { left, operator, right } => {
            let left_val = evaluate(left);
            let right_val = evaluate(right);

            match operator {
                OperatorType::Add => left_val + right_val,
                OperatorType::Subtract => left_val - right_val,
                OperatorType::Multiply => left_val * right_val,
                OperatorType::Divide => left_val / right_val,
            }
        }
    }
}