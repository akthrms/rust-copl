use std::fmt;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Expression {
    Int(i64),
    Bool(bool),
    If(Box<Expression>, Box<Expression>, Box<Expression>),
    Plus(Box<Expression>, Box<Expression>),
    Minus(Box<Expression>, Box<Expression>),
    Times(Box<Expression>, Box<Expression>),
    Lt(Box<Expression>, Box<Expression>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::eval_ml2::ast::Expression::*;

        match self {
            Int(i) => write!(f, "{}", i),
            Bool(b) => write!(f, "{}", b),
            If(expression1, expression2, expression3) => write!(
                f,
                "(if {} then {} else {})",
                expression1, expression2, expression3
            ),
            Plus(expression1, expression2) => write!(f, "({} + {})", expression1, expression2),
            Minus(expression1, expression2) => write!(f, "({} - {})", expression1, expression2),
            Times(expression1, expression2) => write!(f, "({} * {})", expression1, expression2),
            Lt(expression1, expression2) => write!(f, "({} < {})", expression1, expression2),
        }
    }
}
