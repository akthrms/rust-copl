use std::fmt;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Expression {
    Integer(i64),
    Boolean(bool),
    If(Box<Expression>, Box<Expression>, Box<Expression>),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    LessThan(Box<Expression>, Box<Expression>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Expression::*;

        match self {
            Integer(n) => write!(f, "{}", n),
            Boolean(b) => write!(f, "{}", b),
            If(expression1, expression2, expression3) => write!(
                f,
                "if {} then {} else {}",
                expression1, expression2, expression3
            ),
            Add(left, right) => write!(f, "{} + {}", left, right),
            Sub(left, right) => write!(f, "{} - {}", left, right),
            Mul(left, right) => write!(f, "{} * {}", left, right),
            LessThan(left, right) => write!(f, "{} < {}", left, right),
        }
    }
}
