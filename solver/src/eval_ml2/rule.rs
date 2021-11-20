use crate::{
    eval_ml2::ast::{Env, Expr, Expr::*},
    util::ident,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Rule {
    EInt(Env, i64, usize),
    EBool(Env, bool, usize),
    EIfT(Env, Expr, Expr, Expr, Box<Rule>, Box<Rule>, usize),
    EIfF(Env, Expr, Expr, Expr, Box<Rule>, Box<Rule>, usize),
    EPlus(Env, Expr, Expr, Box<Rule>, Box<Rule>, Box<Rule>, usize),
    EMinus(Env, Expr, Expr, Box<Rule>, Box<Rule>, Box<Rule>, usize),
    ETimes(Env, Expr, Expr, Box<Rule>, Box<Rule>, Box<Rule>, usize),
    ELt(Env, Expr, Expr, Box<Rule>, Box<Rule>, Box<Rule>, usize),
    BPlus(Expr, Expr, Expr, usize),
    BMinus(Expr, Expr, Expr, usize),
    BTimes(Expr, Expr, Expr, usize),
    BLt(Expr, Expr, Expr, usize),
    EVar1(Env, Expr, usize),
    EVar2(Env, Expr, Box<Rule>, usize),
    ELet(Env, Expr, Expr, Expr, Box<Rule>, Box<Rule>, usize),
}

impl Rule {
    pub fn evaluated(&self) -> Expr {
        use crate::eval_ml2::rule::Rule::*;

        match self {
            EInt(_, i, _) => Int(i.clone()),
            EBool(_, b, _) => Bool(b.clone()),
            EIfT(_, _, _, _, _, rule2, _) => rule2.evaluated(),
            EIfF(_, _, _, _, _, rule2, _) => rule2.evaluated(),
            EPlus(_, _, _, _, _, rule3, _) => rule3.evaluated(),
            EMinus(_, _, _, _, _, rule3, _) => rule3.evaluated(),
            ETimes(_, _, _, _, _, rule3, _) => rule3.evaluated(),
            ELt(_, _, _, _, _, rule3, _) => rule3.evaluated(),
            BPlus(_, _, expr3, _) => expr3.clone(),
            BMinus(_, _, expr3, _) => expr3.clone(),
            BTimes(_, _, expr3, _) => expr3.clone(),
            BLt(_, _, expr3, _) => expr3.clone(),
            EVar1(env, _, _) => env.last().1,
            EVar2(_, _, rule, _) => rule.evaluated(),
            ELet(_, _, _, _, _, rule2, _) => rule2.evaluated(),
        }
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::eval_ml2::rule::Rule::*;

        match self {
            EInt(env, i, depth) => {
                write!(
                    f,
                    "{}{} |- {} evalto {} by E-Int {{}}",
                    ident(*depth),
                    env,
                    i,
                    i
                )
            }
            EBool(env, b, depth) => {
                write!(
                    f,
                    "{}{} |- {} evalto {} by E-Bool {{}}",
                    ident(*depth),
                    env,
                    b,
                    b
                )
            }
            EIfT(env, expr1, expr2, expr3, rule1, rule2, depth) => {
                writeln!(
                    f,
                    "{}{} |- if {} then {} else {} evalto {} by E-IfT {{",
                    ident(*depth),
                    env,
                    expr1,
                    expr2,
                    expr3,
                    self.evaluated()
                )?;
                writeln!(f, "{};", rule1)?;
                writeln!(f, "{}", rule2)?;
                write!(f, "{}}}", ident(*depth))
            }
            EIfF(env, expr1, expr2, expr3, rule1, rule2, depth) => {
                writeln!(
                    f,
                    "{}{} |- if {} then {} else {} evalto {} by E-IfF {{",
                    ident(*depth),
                    env,
                    expr1,
                    expr2,
                    expr3,
                    self.evaluated()
                )?;
                writeln!(f, "{};", rule1)?;
                writeln!(f, "{}", rule2)?;
                write!(f, "{}}}", ident(*depth))
            }
            EPlus(env, expr1, expr2, rule1, rule2, rule3, depth) => {
                writeln!(
                    f,
                    "{}{} |- {} + {} evalto {} by E-Plus {{",
                    ident(*depth),
                    env,
                    expr1,
                    expr2,
                    self.evaluated()
                )?;
                writeln!(f, "{};", rule1)?;
                writeln!(f, "{};", rule2)?;
                writeln!(f, "{}", rule3)?;
                write!(f, "{}}}", ident(*depth))
            }
            EMinus(env, expr1, expr2, rule1, rule2, rule3, depth) => {
                writeln!(
                    f,
                    "{}{} |- {} - {} evalto {} by E-Minus {{",
                    ident(*depth),
                    env,
                    expr1,
                    expr2,
                    self.evaluated()
                )?;
                writeln!(f, "{};", rule1)?;
                writeln!(f, "{};", rule2)?;
                writeln!(f, "{}", rule3)?;
                write!(f, "{}}}", ident(*depth))
            }
            ETimes(env, expr1, expr2, rule1, rule2, rule3, depth) => {
                writeln!(
                    f,
                    "{}{} |- {} * {} evalto {} by E-Times {{",
                    ident(*depth),
                    env,
                    expr1,
                    expr2,
                    self.evaluated()
                )?;
                writeln!(f, "{};", rule1)?;
                writeln!(f, "{};", rule2)?;
                writeln!(f, "{}", rule3)?;
                write!(f, "{}}}", ident(*depth))
            }
            ELt(env, expr1, expr2, rule1, rule2, rule3, depth) => {
                writeln!(
                    f,
                    "{}{} |- {} < {} evalto {} by E-Lt {{",
                    ident(*depth),
                    env,
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
            EVar1(env, expr, depth) => {
                write!(
                    f,
                    "{}{} |- {} evalto {} by E-Var1 {{}}",
                    ident(*depth),
                    env,
                    expr,
                    self.evaluated()
                )
            }
            EVar2(env, expr, rule, depth) => {
                writeln!(
                    f,
                    "{}{} |- {} evalto {} by E-Var2 {{",
                    ident(*depth),
                    env,
                    expr,
                    self.evaluated()
                )?;
                writeln!(f, "{}", rule)?;
                write!(f, "{}}}", ident(*depth))
            }
            ELet(env, expr1, expr2, expr3, rule1, rule2, depth) => {
                writeln!(
                    f,
                    "{}{} |- let {} = {} in {} evalto {} by E-Let {{",
                    ident(*depth),
                    env,
                    expr1,
                    expr2,
                    expr3,
                    self.evaluated()
                )?;
                writeln!(f, "{};", rule1)?;
                writeln!(f, "{}", rule2)?;
                write!(f, "{}}}", ident(*depth))
            }
        }
    }
}
