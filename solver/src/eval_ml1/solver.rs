use crate::eval_ml1::{ast::Expression, evaluator::evaluate};

pub fn solve(expression: Expression, n: usize, end: bool) {
    use crate::eval_ml1::ast::Expression::*;

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
        If(ref expression1, ref expression2, ref expression3) => {
            match evaluate(*expression1.clone()) {
                Boolean(true) => {
                    println!(
                        "{}{} evalto {} by E-IfT {{",
                        ident(n),
                        expression.clone(),
                        evaluate(expression.clone())
                    );
                    solve(*expression2.clone(), n + 1, true);
                }
                Boolean(false) => {
                    println!(
                        "{}{} evalto {} by E-IfF {{",
                        ident(n),
                        expression.clone(),
                        evaluate(expression.clone())
                    );
                    solve(*expression3.clone(), n + 1, true);
                }
                _ => unreachable!(),
            }
            println!("{}}}{}", ident(n), semicolon(end));
        }
        Add(ref expression1, ref expression2) => {
            println!(
                "{}{} evalto {} by E-Plus {{",
                ident(n),
                expression.clone(),
                evaluate(expression.clone())
            );
            solve(*expression1.clone(), n + 1, false);
            solve(*expression2.clone(), n + 1, false);
            println!(
                "{}{} plus {} is {} by B-Plus {{}}",
                ident(n + 1),
                evaluate(*expression1.clone()),
                evaluate(*expression2.clone()),
                evaluate(expression.clone())
            );
            println!("{}}}{}", ident(n), semicolon(end));
        }
        Sub(ref expression1, ref expression2) => {
            println!(
                "{}{} evalto {} by E-Minus {{",
                ident(n),
                expression.clone(),
                evaluate(expression.clone())
            );
            solve(*expression1.clone(), n + 1, false);
            solve(*expression2.clone(), n + 1, false);
            println!(
                "{}{} minus {} is {} by B-Minus {{}}",
                ident(n + 1),
                evaluate(*expression1.clone()),
                evaluate(*expression2.clone()),
                evaluate(expression.clone())
            );
            println!("{}}}{}", ident(n), semicolon(end));
        }
        Mul(ref expression1, ref expression2) => {
            println!(
                "{}{} evalto {} by E-Times {{",
                ident(n),
                expression.clone(),
                evaluate(expression.clone())
            );
            solve(*expression1.clone(), n + 1, false);
            solve(*expression2.clone(), n + 1, false);
            println!(
                "{}{} times {} is {} by B-Times {{}}",
                ident(n + 1),
                evaluate(*expression1.clone()),
                evaluate(*expression2.clone()),
                evaluate(expression.clone())
            );
            println!("{}}}{}", ident(n), semicolon(end));
        }
        LessThan(ref expression1, ref expression2) => {
            println!(
                "{}{} evalto {} by E-Lt {{",
                ident(n),
                expression.clone(),
                evaluate(expression.clone())
            );
            solve(*expression1.clone(), n + 1, false);
            solve(*expression2.clone(), n + 1, false);
            println!(
                "{}{} is less than {} by B-Lt {{}}",
                ident(n + 1),
                evaluate(*expression1.clone()),
                evaluate(*expression2.clone()),
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
