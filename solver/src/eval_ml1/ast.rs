use std::fmt;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Expr {
    Int(i64),
    Bool(bool),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Times(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::eval_ml1::ast::Expr::*;

        match self {
            Int(i) => write!(f, "{}", i),
            Bool(b) => write!(f, "{}", b),
            If(expr1, expr2, expr3) => write!(f, "(if {} then {} else {})", expr1, expr2, expr3),
            Plus(expr1, expr2) => write!(f, "({} + {})", expr1, expr2),
            Minus(expr1, expr2) => write!(f, "({} - {})", expr1, expr2),
            Times(expr1, expr2) => write!(f, "({} * {})", expr1, expr2),
            Lt(expr1, expr2) => write!(f, "({} < {})", expr1, expr2),
        }
    }
}
