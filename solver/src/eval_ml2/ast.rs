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
    Let(Box<Expression>, Box<Expression>, Box<Expression>),
    Var(String),
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
            Let(expression1, expression2, expression3) => {
                write!(
                    f,
                    "(let {} = {} in {})",
                    expression1, expression2, expression3
                )
            }
            Var(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Environment(Vec<(Expression, Expression)>);

impl Environment {
    pub fn new(pairs: Vec<(Expression, Expression)>) -> Environment {
        Environment(pairs)
    }

    pub fn empty() -> Environment {
        Environment(vec![])
    }

    pub fn unwrap(&self) -> Vec<(Expression, Expression)> {
        self.0.clone()
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vars = self
            .unwrap()
            .into_iter()
            .map(|(expression1, expression2)| format!("{} = {}", expression1, expression2))
            .collect::<Vec<String>>();
        write!(f, "{}", vars.join(", "))
    }
}
