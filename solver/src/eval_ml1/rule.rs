use crate::{
    eval_ml1::ast::{Expression, Expression::*},
    util::ident,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Rule {
    EInt(i64, usize),
    EBool(bool, usize),
    EIfT(
        Expression,
        Expression,
        Expression,
        Box<Rule>,
        Box<Rule>,
        usize,
    ),
    EIfF(
        Expression,
        Expression,
        Expression,
        Box<Rule>,
        Box<Rule>,
        usize,
    ),
    EPlus(
        Expression,
        Expression,
        Box<Rule>,
        Box<Rule>,
        Box<Rule>,
        usize,
    ),
    EMinus(
        Expression,
        Expression,
        Box<Rule>,
        Box<Rule>,
        Box<Rule>,
        usize,
    ),
    ETimes(
        Expression,
        Expression,
        Box<Rule>,
        Box<Rule>,
        Box<Rule>,
        usize,
    ),
    ELt(
        Expression,
        Expression,
        Box<Rule>,
        Box<Rule>,
        Box<Rule>,
        usize,
    ),
    BPlus(Expression, Expression, Expression, usize),
    BMinus(Expression, Expression, Expression, usize),
    BTimes(Expression, Expression, Expression, usize),
    BLt(Expression, Expression, Expression, usize),
}

impl Rule {
    pub fn evaluated(&self) -> Expression {
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
            BPlus(_, _, expression3, _) => expression3.clone(),
            BMinus(_, _, expression3, _) => expression3.clone(),
            BTimes(_, _, expression3, _) => expression3.clone(),
            BLt(_, _, expression3, _) => expression3.clone(),
        }
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::eval_ml1::rule::Rule::*;

        match self {
            EInt(i, depth) => write!(f, "{}{} evalto {} by E-Int {{}}", ident(*depth), i, i),
            EBool(b, depth) => write!(f, "{}{} evalto {} by E-Bool {{}}", ident(*depth), b, b),
            EIfT(expression1, expression2, expression3, rule1, rule2, depth) => {
                writeln!(
                    f,
                    "{}if {} then {} else {} evalto {} by E-IfT {{",
                    ident(*depth),
                    expression1,
                    expression2,
                    expression3,
                    self.evaluated()
                )?;
                writeln!(f, "{};", rule1)?;
                writeln!(f, "{}", rule2)?;
                write!(f, "{}}}", ident(*depth))
            }
            EIfF(expression1, expression2, expression3, rule1, rule2, depth) => {
                writeln!(
                    f,
                    "{}if {} then {} else {} evalto {} by E-IfF {{",
                    ident(*depth),
                    expression1,
                    expression2,
                    expression3,
                    self.evaluated()
                )?;
                writeln!(f, "{};", rule1)?;
                writeln!(f, "{}", rule2)?;
                write!(f, "{}}}", ident(*depth))
            }
            EPlus(expression1, expression2, rule1, rule2, rule3, depth) => {
                writeln!(
                    f,
                    "{}{} + {} evalto {} by E-Plus {{",
                    ident(*depth),
                    expression1,
                    expression2,
                    self.evaluated()
                )?;
                writeln!(f, "{};", rule1)?;
                writeln!(f, "{};", rule2)?;
                writeln!(f, "{}", rule3)?;
                write!(f, "{}}}", ident(*depth))
            }
            EMinus(expression1, expression2, rule1, rule2, rule3, depth) => {
                writeln!(
                    f,
                    "{}{} - {} evalto {} by E-Minus {{",
                    ident(*depth),
                    expression1,
                    expression2,
                    self.evaluated()
                )?;
                writeln!(f, "{};", rule1)?;
                writeln!(f, "{};", rule2)?;
                writeln!(f, "{}", rule3)?;
                write!(f, "{}}}", ident(*depth))
            }
            ETimes(expression1, expression2, rule1, rule2, rule3, depth) => {
                writeln!(
                    f,
                    "{}{} * {} evalto {} by E-Times {{",
                    ident(*depth),
                    expression1,
                    expression2,
                    self.evaluated()
                )?;
                writeln!(f, "{};", rule1)?;
                writeln!(f, "{};", rule2)?;
                writeln!(f, "{}", rule3)?;
                write!(f, "{}}}", ident(*depth))
            }
            ELt(expression1, expression2, rule1, rule2, rule3, depth) => {
                writeln!(
                    f,
                    "{}{} < {} evalto {} by E-Lt {{",
                    ident(*depth),
                    expression1,
                    expression2,
                    self.evaluated()
                )?;
                writeln!(f, "{};", rule1)?;
                writeln!(f, "{};", rule2)?;
                writeln!(f, "{}", rule3)?;
                write!(f, "{}}}", ident(*depth))
            }
            BPlus(expression1, expression2, expression3, depth) => {
                write!(
                    f,
                    "{}{} plus {} is {} by B-Plus {{}}",
                    ident(*depth),
                    expression1,
                    expression2,
                    expression3
                )
            }
            BMinus(expression1, expression2, expression3, depth) => {
                write!(
                    f,
                    "{}{} minus {} is {} by B-Minus {{}}",
                    ident(*depth),
                    expression1,
                    expression2,
                    expression3
                )
            }
            BTimes(expression1, expression2, expression3, depth) => {
                write!(
                    f,
                    "{}{} times {} is {} by B-Times {{}}",
                    ident(*depth),
                    expression1,
                    expression2,
                    expression3
                )
            }
            BLt(expression1, expression2, _, depth) => {
                write!(
                    f,
                    "{}{} is less than {} by B-Lt {{}}",
                    ident(*depth),
                    expression1,
                    expression2,
                )
            }
        }
    }
}
