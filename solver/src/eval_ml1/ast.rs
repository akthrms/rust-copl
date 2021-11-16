use std::fmt;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Expression {
    Integer(i64),
    Boolean(bool),
    If(Box<Expression>, Box<Expression>, Box<Expression>),
    Plus(Box<Expression>, Box<Expression>),
    Minus(Box<Expression>, Box<Expression>),
    Times(Box<Expression>, Box<Expression>),
    LessThan(Box<Expression>, Box<Expression>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::eval_ml1::ast::Expression::*;

        match self {
            Integer(n) => write!(f, "{}", n),
            Boolean(b) => write!(f, "{}", b),
            If(condition, consequence, alternative) => write!(
                f,
                "if {} then {} else {}",
                condition, consequence, alternative
            ),
            Plus(left, right) => write!(f, "{} + {}", left, right),
            Minus(left, right) => write!(f, "{} - {}", left, right),
            Times(left, right) => write!(f, "{} * {}", left, right),
            LessThan(left, right) => write!(f, "{} < {}", left, right),
        }
    }
}
