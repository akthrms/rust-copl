use crate::{
    eval_ml1::ast::{Expr, Expr::*},
    util::ident,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Rule {
    EInt(i64, usize),
    EBool(bool, usize),
    EIfT(Expr, Expr, Expr, Box<Rule>, Box<Rule>, usize),
    EIfF(Expr, Expr, Expr, Box<Rule>, Box<Rule>, usize),
    EPlus(Expr, Expr, Box<Rule>, Box<Rule>, Box<Rule>, usize),
    EMinus(Expr, Expr, Box<Rule>, Box<Rule>, Box<Rule>, usize),
    ETimes(Expr, Expr, Box<Rule>, Box<Rule>, Box<Rule>, usize),
    ELt(Expr, Expr, Box<Rule>, Box<Rule>, Box<Rule>, usize),
    BPlus(Expr, Expr, Expr, usize),
    BMinus(Expr, Expr, Expr, usize),
    BTimes(Expr, Expr, Expr, usize),
    BLt(Expr, Expr, Expr, usize),
}

impl Rule {
    pub fn evaluated(&self) -> Expr {
        use crate::eval_ml1::rule::Rule::*;

        match self {
            EInt(i, _) => Int(i.clone()),
            EBool(b, _) => Bool(b.clone()),
            EIfT(_, _, _, _, rule2, _) => rule2.evaluated(),
            EIfF(_, _, _, _, rule2, _) => rule2.evaluated(),
            EPlus(_, _, _, _, rule3, _) => rule3.evaluated(),
            EMinus(_, _, _, _, rule3, _) => rule3.evaluated(),
            ETimes(_, _, _, _, rule3, _) => rule3.evaluated(),
            ELt(_, _, _, _, rule3, _) => rule3.evaluated(),
            BPlus(_, _, expr3, _) => expr3.clone(),
            BMinus(_, _, expr3, _) => expr3.clone(),
            BTimes(_, _, expr3, _) => expr3.clone(),
            BLt(_, _, expr3, _) => expr3.clone(),
        }
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::eval_ml1::rule::Rule::*;

        match self {
            EInt(i, depth) => write!(f, "{}{} evalto {} by E-Int {{}}", ident(*depth), i, i),
            EBool(b, depth) => write!(f, "{}{} evalto {} by E-Bool {{}}", ident(*depth), b, b),
            EIfT(expr1, expr2, expr3, rule1, rule2, depth) => {
                writeln!(
                    f,
                    "{}if {} then {} else {} evalto {} by E-IfT {{",
                    ident(*depth),
                    expr1,
                    expr2,
                    expr3,
                    self.evaluated()
                )?;
                writeln!(f, "{};", rule1)?;
                writeln!(f, "{}", rule2)?;
                write!(f, "{}}}", ident(*depth))
            }
            EIfF(expr1, expr2, expr3, rule1, rule2, depth) => {
                writeln!(
                    f,
                    "{}if {} then {} else {} evalto {} by E-IfF {{",
                    ident(*depth),
                    expr1,
                    expr2,
                    expr3,
                    self.evaluated()
                )?;
                writeln!(f, "{};", rule1)?;
                writeln!(f, "{}", rule2)?;
                write!(f, "{}}}", ident(*depth))
            }
            EPlus(expr1, expr2, rule1, rule2, rule3, depth) => {
                writeln!(
                    f,
                    "{}{} + {} evalto {} by E-Plus {{",
                    ident(*depth),
                    expr1,
                    expr2,
                    self.evaluated()
                )?;
                writeln!(f, "{};", rule1)?;
                writeln!(f, "{};", rule2)?;
                writeln!(f, "{}", rule3)?;
                write!(f, "{}}}", ident(*depth))
            }
            EMinus(expr1, expr2, rule1, rule2, rule3, depth) => {
                writeln!(
                    f,
                    "{}{} - {} evalto {} by E-Minus {{",
                    ident(*depth),
                    expr1,
                    expr2,
                    self.evaluated()
                )?;
                writeln!(f, "{};", rule1)?;
                writeln!(f, "{};", rule2)?;
                writeln!(f, "{}", rule3)?;
                write!(f, "{}}}", ident(*depth))
            }
            ETimes(expr1, expr2, rule1, rule2, rule3, depth) => {
                writeln!(
                    f,
                    "{}{} * {} evalto {} by E-Times {{",
                    ident(*depth),
                    expr1,
                    expr2,
                    self.evaluated()
                )?;
                writeln!(f, "{};", rule1)?;
                writeln!(f, "{};", rule2)?;
                writeln!(f, "{}", rule3)?;
                write!(f, "{}}}", ident(*depth))
            }
            ELt(expr1, expr2, rule1, rule2, rule3, depth) => {
                writeln!(
                    f,
                    "{}{} < {} evalto {} by E-Lt {{",
                    ident(*depth),
                    expr1,
                    expr2,
                    self.evaluated()
                )?;
                writeln!(f, "{};", rule1)?;
                writeln!(f, "{};", rule2)?;
                writeln!(f, "{}", rule3)?;
                write!(f, "{}}}", ident(*depth))
            }
            BPlus(expr1, expr2, expr3, depth) => {
                write!(
                    f,
                    "{}{} plus {} is {} by B-Plus {{}}",
                    ident(*depth),
                    expr1,
                    expr2,
                    expr3
                )
            }
            BMinus(expr1, expr2, expr3, depth) => {
                write!(
                    f,
                    "{}{} minus {} is {} by B-Minus {{}}",
                    ident(*depth),
                    expr1,
                    expr2,
                    expr3
                )
            }
            BTimes(expr1, expr2, expr3, depth) => {
                write!(
                    f,
                    "{}{} times {} is {} by B-Times {{}}",
                    ident(*depth),
                    expr1,
                    expr2,
                    expr3
                )
            }
            BLt(expr1, expr2, _, depth) => {
                write!(
                    f,
                    "{}{} is less than {} by B-Lt {{}}",
                    ident(*depth),
                    expr1,
                    expr2,
                )
            }
        }
    }
}
