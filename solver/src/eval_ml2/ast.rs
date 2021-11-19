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
    pub fn new() -> Environment {
        Environment(vec![])
    }

    pub fn put(&mut self, expression1: Expression, expression2: Expression) {
        self.0.insert(0, (expression1, expression2))
    }

    pub fn get(&self, expression: &Expression) -> Expression {
        self.0
            .iter()
            .find(|(expression1, _)| expression1 == expression)
            .unwrap()
            .1
            .clone()
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pairs = self
            .0
            .iter()
            .rev()
            .map(|(expression1, expression2)| format!("{} = {}", expression1, expression2))
            .collect::<Vec<String>>();
        write!(f, "{}", pairs.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use crate::eval_ml2::ast::{Environment, Expression::*};

    #[test]
    fn test_environment() {
        let mut environment = Environment::new();
        environment.put(Var("x".to_string()), Int(1));
        environment.put(Var("y".to_string()), Int(2));
        environment.put(Var("x".to_string()), Int(3));
        assert_eq!("x = 1, y = 2, x = 3", environment.to_string());
        assert_eq!(Int(3), environment.get(&Var("x".to_string())));
        assert_eq!(Int(2), environment.get(&Var("y".to_string())));
    }
}
