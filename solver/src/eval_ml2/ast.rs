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
    Let(Box<Expr>, Box<Expr>, Box<Expr>),
    Var(String),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::eval_ml2::ast::Expr::*;

        match self {
            Int(i) => write!(f, "{}", i),
            Bool(b) => write!(f, "{}", b),
            If(expr1, expr2, expr3) => write!(f, "(if {} then {} else {})", expr1, expr2, expr3),
            Plus(expr1, expr2) => write!(f, "({} + {})", expr1, expr2),
            Minus(expr1, expr2) => write!(f, "({} - {})", expr1, expr2),
            Times(expr1, expr2) => write!(f, "({} * {})", expr1, expr2),
            Lt(expr1, expr2) => write!(f, "({} < {})", expr1, expr2),
            Let(expr1, expr2, expr3) => {
                write!(f, "(let {} = {} in {})", expr1, expr2, expr3)
            }
            Var(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Env(Vec<(Expr, Expr)>);

impl Env {
    pub fn new() -> Env {
        Env(vec![])
    }

    pub fn put(&mut self, expr1: Expr, expr2: Expr) {
        self.0.insert(0, (expr1, expr2))
    }

    pub fn get(&self, expr: &Expr) -> Expr {
        self.0
            .iter()
            .find(|(expr1, _)| expr1 == expr)
            .unwrap()
            .1
            .clone()
    }
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pairs = self
            .0
            .iter()
            .rev()
            .map(|(expr1, expr2)| format!("{} = {}", expr1, expr2))
            .collect::<Vec<String>>();
        write!(f, "{}", pairs.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use crate::eval_ml2::ast::{Env, Expr::*};

    #[test]
    fn test_env() {
        let mut env = Env::new();
        env.put(Var("x".to_string()), Int(1));
        env.put(Var("y".to_string()), Int(2));
        env.put(Var("x".to_string()), Int(3));
        assert_eq!("x = 1, y = 2, x = 3", env.to_string());
        assert_eq!(Int(3), env.get(&Var("x".to_string())));
        assert_eq!(Int(2), env.get(&Var("y".to_string())));
    }
}
