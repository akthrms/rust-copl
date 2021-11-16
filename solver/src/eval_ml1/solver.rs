use crate::eval_ml1::{
    ast::{Expression, Expression::*},
    evaluator::evaluate,
};

pub fn solve(expression: Expression, n: usize, end: bool) {
    match expression {
        Integer(_) => println!(
            "{}{} evalto {} by E-Int {{}}{}",
            ident(n),
            expression.clone(),
            evaluate(expression.clone()),
            semicolon(end)
        ),
        Boolean(_) => println!(
            "{}{} evalto {} by E-Bool {{}}{}",
            ident(n),
            expression.clone(),
            evaluate(expression.clone()),
            semicolon(end)
        ),
        If(ref condition, ref consequence, ref alternative) => {
            match evaluate(*condition.clone()) {
                Boolean(true) => {
                    println!(
                        "{}{} evalto {} by E-IfT {{",
                        ident(n),
                        expression.clone(),
                        evaluate(expression.clone())
                    );
                    solve(*condition.clone(), n + 1, false);
                    solve(*consequence.clone(), n + 1, true);
                }
                Boolean(false) => {
                    println!(
                        "{}{} evalto {} by E-IfF {{",
                        ident(n),
                        expression.clone(),
                        evaluate(expression.clone())
                    );
                    solve(*condition.clone(), n + 1, false);
                    solve(*alternative.clone(), n + 1, true);
                }
                _ => unreachable!(),
            }
            println!("{}}}{}", ident(n), semicolon(end));
        }
        Plus(ref left, ref right) => {
            println!(
                "{}{} evalto {} by E-Plus {{",
                ident(n),
                expression.clone(),
                evaluate(expression.clone())
            );
            solve(*left.clone(), n + 1, false);
            solve(*right.clone(), n + 1, false);
            println!(
                "{}{} plus {} is {} by B-Plus {{}}",
                ident(n + 1),
                evaluate(*left.clone()),
                evaluate(*right.clone()),
                evaluate(expression.clone())
            );
            println!("{}}}{}", ident(n), semicolon(end));
        }
        Minus(ref left, ref right) => {
            println!(
                "{}{} evalto {} by E-Minus {{",
                ident(n),
                expression.clone(),
                evaluate(expression.clone())
            );
            solve(*left.clone(), n + 1, false);
            solve(*right.clone(), n + 1, false);
            println!(
                "{}{} minus {} is {} by B-Minus {{}}",
                ident(n + 1),
                evaluate(*left.clone()),
                evaluate(*right.clone()),
                evaluate(expression.clone())
            );
            println!("{}}}{}", ident(n), semicolon(end));
        }
        Times(ref left, ref right) => {
            println!(
                "{}{} evalto {} by E-Times {{",
                ident(n),
                expression.clone(),
                evaluate(expression.clone())
            );
            solve(*left.clone(), n + 1, false);
            solve(*right.clone(), n + 1, false);
            println!(
                "{}{} times {} is {} by B-Times {{}}",
                ident(n + 1),
                evaluate(*left.clone()),
                evaluate(*right.clone()),
                evaluate(expression.clone())
            );
            println!("{}}}{}", ident(n), semicolon(end));
        }
        LessThan(ref left, ref right) => {
            println!(
                "{}{} evalto {} by E-Lt {{",
                ident(n),
                expression.clone(),
                evaluate(expression.clone())
            );
            solve(*left.clone(), n + 1, false);
            solve(*right.clone(), n + 1, false);
            println!(
                "{}{} is less than {} by B-Lt {{}}",
                ident(n + 1),
                evaluate(*left.clone()),
                evaluate(*right.clone()),
            );
            println!("{}}}{}", ident(n), semicolon(end));
        }
    }
}

fn ident(n: usize) -> String {
    "    ".repeat(n)
}

fn semicolon(end: bool) -> String {
    (if end { "" } else { ";" }).to_string()
}
