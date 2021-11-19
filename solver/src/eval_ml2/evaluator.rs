use crate::eval_ml2::ast::{Env, Expr, Expr::*};

pub fn eval(env: &Env, expr: &Expr) -> Expr {
    match expr {
        Int(i) => Int(i.clone()),
        Bool(b) => Bool(b.clone()),
        If(expr1, expr2, expr3) => match eval(env, expr1) {
            Bool(true) => eval(env, expr2),
            Bool(false) => eval(env, expr3),
            _ => unreachable!(),
        },
        Plus(expr1, expr2) => match (eval(env, expr1), eval(env, expr2)) {
            (Int(i1), Int(i2)) => Int(i1 + i2),
            _ => unreachable!(),
        },
        Minus(expr1, expr2) => match (eval(env, expr1), eval(env, expr2)) {
            (Int(i1), Int(i2)) => Int(i1 - i2),
            _ => unreachable!(),
        },
        Times(expr1, expr2) => match (eval(env, expr1), eval(env, expr2)) {
            (Int(i1), Int(i2)) => Int(i1 * i2),
            _ => unreachable!(),
        },
        Lt(expr1, expr2) => match (eval(env, expr1), eval(env, expr2)) {
            (Int(i1), Int(i2)) => Bool(i1 < i2),
            _ => unreachable!(),
        },
        Let(expr1, expr2, expr3) => {
            let expr1 = *expr1.clone();
            let expr2 = eval(env, expr2);
            let mut env = env.clone();
            env.put(expr1, expr2);
            eval(&env, expr3)
        }
        Var(_) => env.get(expr),
    }
}

#[cfg(test)]
mod tests {
    use crate::eval_ml2::{
        ast::{Env, Expr::*},
        evaluator::eval,
    };

    #[test]
    fn test_eval1() {
        assert_eq!(
            eval(&Env::new(), &Plus(Box::new(Int(3)), Box::new(Int(5)))),
            Int(8)
        );
    }

    #[test]
    fn test_eval2() {
        assert_eq!(
            eval(
                &Env::new(),
                &Minus(
                    Box::new(Minus(Box::new(Int(8)), Box::new(Int(2)))),
                    Box::new(Int(3))
                )
            ),
            Int(3)
        );
    }

    #[test]
    fn test_eval3() {
        assert_eq!(
            eval(
                &Env::new(),
                &Times(
                    Box::new(Plus(Box::new(Int(4)), Box::new(Int(5)))),
                    Box::new(Minus(Box::new(Int(1)), Box::new(Int(10))))
                )
            ),
            Int(-81)
        );
    }

    #[test]
    fn test_eval4() {
        assert_eq!(
            eval(
                &Env::new(),
                &If(
                    Box::new(Lt(Box::new(Int(4)), Box::new(Int(5)))),
                    Box::new(Plus(Box::new(Int(2)), Box::new(Int(3)))),
                    Box::new(Times(Box::new(Int(8)), Box::new(Int(8))))
                )
            ),
            Int(5)
        );
    }

    #[test]
    fn test_eval5() {
        assert_eq!(
            eval(
                &Env::new(),
                &Plus(
                    Box::new(Int(3)),
                    Box::new(If(
                        Box::new(Lt(
                            Box::new(Int(-23)),
                            Box::new(Times(Box::new(Int(-2)), Box::new(Int(8))))
                        )),
                        Box::new(Int(8)),
                        Box::new(Plus(Box::new(Int(2)), Box::new(Int(4))))
                    ))
                )
            ),
            Int(11)
        );
    }

    #[test]
    fn test_eval6() {
        assert_eq!(
            eval(
                &Env::new(),
                &Plus(
                    Box::new(Plus(
                        Box::new(Int(3)),
                        Box::new(If(
                            Box::new(Lt(
                                Box::new(Int(-23)),
                                Box::new(Times(Box::new(Int(-2)), Box::new(Int(8))))
                            )),
                            Box::new(Int(8)),
                            Box::new(Int(2))
                        ))
                    )),
                    Box::new(Int(4))
                )
            ),
            Int(15)
        );
    }

    #[test]
    fn test_eval7() {
        let mut env = Env::new();
        env.put(Var("x".to_string()), Int(3));
        env.put(Var("y".to_string()), Int(2));
        assert_eq!(eval(&env, &Var("x".to_string())), Int(3));
    }

    #[test]
    fn test_eval8() {
        let mut env = Env::new();
        env.put(Var("x".to_string()), Bool(true));
        env.put(Var("y".to_string()), Int(4));
        assert_eq!(
            eval(
                &env,
                &If(
                    Box::new(Var("x".to_string())),
                    Box::new(Plus(Box::new(Var("y".to_string())), Box::new(Int(1)))),
                    Box::new(Var("y".to_string()))
                )
            ),
            Int(5)
        );
    }

    #[test]
    fn test_eval9() {
        assert_eq!(
            eval(
                &Env::new(),
                &Let(
                    Box::new(Var("x".to_string())),
                    Box::new(Plus(Box::new(Int(1)), Box::new(Int(2)))),
                    Box::new(Times(Box::new(Var("x".to_string())), Box::new(Int(4))))
                )
            ),
            Int(12)
        );
    }

    #[test]
    fn test_eval10() {
        assert_eq!(
            eval(
                &Env::new(),
                &Let(
                    Box::new(Var("x".to_string())),
                    Box::new(Times(Box::new(Int(3)), Box::new(Int(3)))),
                    Box::new(Let(
                        Box::new(Var("y".to_string())),
                        Box::new(Times(Box::new(Int(4)), Box::new(Var("x".to_string())))),
                        Box::new(Plus(
                            Box::new(Var("x".to_string())),
                            Box::new(Var("y".to_string()))
                        ))
                    ))
                )
            ),
            Int(45)
        );
    }

    #[test]
    fn test_eval11() {
        let mut env = Env::new();
        env.put(Var("x".to_string()), Int(3));
        assert_eq!(
            eval(
                &env,
                &Let(
                    Box::new(Var("x".to_string())),
                    Box::new(Times(Box::new(Var("x".to_string())), Box::new(Int(2)))),
                    Box::new(Plus(
                        Box::new(Var("x".to_string())),
                        Box::new(Var("x".to_string()))
                    )),
                )
            ),
            Int(12)
        );
    }
}
