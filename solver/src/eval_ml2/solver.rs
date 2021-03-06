use crate::eval_ml2::{
    ast::{Env, Expr, Expr::*},
    rule::{Rule, Rule::*},
};

pub fn solve(env: &Env, expr: &Expr, depth: usize) -> Rule {
    match expr {
        Int(i) => EInt(env.clone(), i.clone(), depth),
        Bool(b) => EBool(env.clone(), b.clone(), depth),
        If(expr1, expr2, expr3) => {
            let rule1 = solve(env, expr1, depth + 1);
            match rule1.evaluated() {
                Bool(true) => {
                    let rule2 = solve(env, expr2, depth + 1);
                    EIfT(
                        env.clone(),
                        *expr1.clone(),
                        *expr2.clone(),
                        *expr3.clone(),
                        Box::new(rule1),
                        Box::new(rule2),
                        depth,
                    )
                }
                Bool(false) => {
                    let rule2 = solve(env, expr3, depth + 1);
                    EIfF(
                        env.clone(),
                        *expr1.clone(),
                        *expr2.clone(),
                        *expr3.clone(),
                        Box::new(rule1),
                        Box::new(rule2),
                        depth,
                    )
                }
                _ => unreachable!(),
            }
        }
        Plus(expr1, expr2) => {
            let rule1 = solve(env, expr1, depth + 1);
            let rule2 = solve(env, expr2, depth + 1);
            let expr3 = match (rule1.evaluated(), rule2.evaluated()) {
                (Int(i1), Int(i2)) => Int(i1 + i2),
                _ => unreachable!(),
            };
            let rule3 = BPlus(rule1.evaluated(), rule2.evaluated(), expr3, depth + 1);
            EPlus(
                env.clone(),
                *expr1.clone(),
                *expr2.clone(),
                Box::new(rule1),
                Box::new(rule2),
                Box::new(rule3),
                depth,
            )
        }
        Minus(expr1, expr2) => {
            let rule1 = solve(env, expr1, depth + 1);
            let rule2 = solve(env, expr2, depth + 1);
            let expr3 = match (rule1.evaluated(), rule2.evaluated()) {
                (Int(i1), Int(i2)) => Int(i1 - i2),
                _ => unreachable!(),
            };
            let rule3 = BMinus(rule1.evaluated(), rule2.evaluated(), expr3, depth + 1);
            EMinus(
                env.clone(),
                *expr1.clone(),
                *expr2.clone(),
                Box::new(rule1),
                Box::new(rule2),
                Box::new(rule3),
                depth,
            )
        }
        Times(expr1, expr2) => {
            let rule1 = solve(env, expr1, depth + 1);
            let rule2 = solve(env, expr2, depth + 1);
            let expr3 = match (rule1.evaluated(), rule2.evaluated()) {
                (Int(i1), Int(i2)) => Int(i1 * i2),
                _ => unreachable!(),
            };
            let rule3 = BTimes(rule1.evaluated(), rule2.evaluated(), expr3, depth + 1);
            ETimes(
                env.clone(),
                *expr1.clone(),
                *expr2.clone(),
                Box::new(rule1),
                Box::new(rule2),
                Box::new(rule3),
                depth,
            )
        }
        Lt(expr1, expr2) => {
            let rule1 = solve(env, expr1, depth + 1);
            let rule2 = solve(env, expr2, depth + 1);
            let expr3 = match (rule1.evaluated(), rule2.evaluated()) {
                (Int(i1), Int(i2)) => Bool(i1 < i2),
                _ => unreachable!(),
            };
            let rule3 = BLt(rule1.evaluated(), rule2.evaluated(), expr3, depth + 1);
            ELt(
                env.clone(),
                *expr1.clone(),
                *expr2.clone(),
                Box::new(rule1),
                Box::new(rule2),
                Box::new(rule3),
                depth,
            )
        }
        Var(_) => {
            if env.last().0 == expr.clone() {
                EVar1(env.clone(), expr.clone(), depth)
            } else {
                let rule = solve(&env.butlast(), expr, depth + 1);
                EVar2(env.clone(), expr.clone(), Box::new(rule), depth)
            }
        }
        Let(expr1, expr2, expr3) => {
            let rule1 = solve(env, expr2, depth + 1);
            let mut new_env = env.clone();
            new_env.put(*expr1.clone(), rule1.evaluated());
            let rule2 = solve(&new_env, expr3, depth + 1);
            ELet(
                env.clone(),
                *expr1.clone(),
                *expr2.clone(),
                *expr3.clone(),
                Box::new(rule1),
                Box::new(rule2),
                depth,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::eval_ml2::{
        ast::{Env, Expr::*},
        rule::Rule::*,
        solver::solve,
    };

    #[test]
    fn test_solve1() {
        assert_eq!(
            solve(&Env::new(), &Plus(Box::new(Int(3)), Box::new(Int(5))), 0),
            EPlus(
                Env::new(),
                Int(3),
                Int(5),
                Box::new(EInt(Env::new(), 3, 1)),
                Box::new(EInt(Env::new(), 5, 1)),
                Box::new(BPlus(Int(3), Int(5), Int(8), 1)),
                0
            )
        );
    }

    #[test]
    fn test_solve2() {
        assert_eq!(
            solve(
                &Env::new(),
                &Minus(
                    Box::new(Minus(Box::new(Int(8)), Box::new(Int(2)))),
                    Box::new(Int(3))
                ),
                0
            ),
            EMinus(
                Env::new(),
                Minus(Box::new(Int(8)), Box::new(Int(2))),
                Int(3),
                Box::new(EMinus(
                    Env::new(),
                    Int(8),
                    Int(2),
                    Box::new(EInt(Env::new(), 8, 2)),
                    Box::new(EInt(Env::new(), 2, 2)),
                    Box::new(BMinus(Int(8), Int(2), Int(6), 2)),
                    1
                )),
                Box::new(EInt(Env::new(), 3, 1)),
                Box::new(BMinus(Int(6), Int(3), Int(3), 1)),
                0
            )
        );
    }

    #[test]
    fn test_solve3() {
        assert_eq!(
            solve(
                &Env::new(),
                &Times(
                    Box::new(Plus(Box::new(Int(4)), Box::new(Int(5)))),
                    Box::new(Minus(Box::new(Int(1)), Box::new(Int(10))))
                ),
                0
            ),
            ETimes(
                Env::new(),
                Plus(Box::new(Int(4)), Box::new(Int(5))),
                Minus(Box::new(Int(1)), Box::new(Int(10))),
                Box::new(EPlus(
                    Env::new(),
                    Int(4),
                    Int(5),
                    Box::new(EInt(Env::new(), 4, 2)),
                    Box::new(EInt(Env::new(), 5, 2)),
                    Box::new(BPlus(Int(4), Int(5), Int(9), 2)),
                    1
                )),
                Box::new(EMinus(
                    Env::new(),
                    Int(1),
                    Int(10),
                    Box::new(EInt(Env::new(), 1, 2)),
                    Box::new(EInt(Env::new(), 10, 2)),
                    Box::new(BMinus(Int(1), Int(10), Int(-9), 2)),
                    1
                )),
                Box::new(BTimes(Int(9), Int(-9), Int(-81), 1)),
                0
            )
        );
    }

    #[test]
    fn test_solve4() {
        assert_eq!(
            solve(
                &Env::new(),
                &If(
                    Box::new(Lt(Box::new(Int(4)), Box::new(Int(5)))),
                    Box::new(Plus(Box::new(Int(2)), Box::new(Int(3)))),
                    Box::new(Times(Box::new(Int(8)), Box::new(Int(8))))
                ),
                0
            ),
            EIfT(
                Env::new(),
                Lt(Box::new(Int(4)), Box::new(Int(5))),
                Plus(Box::new(Int(2)), Box::new(Int(3))),
                Times(Box::new(Int(8)), Box::new(Int(8))),
                Box::new(ELt(
                    Env::new(),
                    Int(4),
                    Int(5),
                    Box::new(EInt(Env::new(), 4, 2)),
                    Box::new(EInt(Env::new(), 5, 2)),
                    Box::new(BLt(Int(4), Int(5), Bool(true), 2)),
                    1
                )),
                Box::new(EPlus(
                    Env::new(),
                    Int(2),
                    Int(3),
                    Box::new(EInt(Env::new(), 2, 2)),
                    Box::new(EInt(Env::new(), 3, 2)),
                    Box::new(BPlus(Int(2), Int(3), Int(5), 2)),
                    1
                )),
                0
            )
        );
    }

    #[test]
    fn test_solve5() {
        assert_eq!(
            solve(
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
                ),
                0
            ),
            EPlus(
                Env::new(),
                Int(3),
                If(
                    Box::new(Lt(
                        Box::new(Int(-23)),
                        Box::new(Times(Box::new(Int(-2)), Box::new(Int(8))))
                    )),
                    Box::new(Int(8)),
                    Box::new(Plus(Box::new(Int(2)), Box::new(Int(4))))
                ),
                Box::new(EInt(Env::new(), 3, 1)),
                Box::new(EIfT(
                    Env::new(),
                    Lt(
                        Box::new(Int(-23)),
                        Box::new(Times(Box::new(Int(-2)), Box::new(Int(8))))
                    ),
                    Int(8),
                    Plus(Box::new(Int(2)), Box::new(Int(4))),
                    Box::new(ELt(
                        Env::new(),
                        Int(-23),
                        Times(Box::new(Int(-2)), Box::new(Int(8))),
                        Box::new(EInt(Env::new(), -23, 3)),
                        Box::new(ETimes(
                            Env::new(),
                            Int(-2),
                            Int(8),
                            Box::new(EInt(Env::new(), -2, 4)),
                            Box::new(EInt(Env::new(), 8, 4)),
                            Box::new(BTimes(Int(-2), Int(8), Int(-16), 4)),
                            3
                        )),
                        Box::new(BLt(Int(-23), Int(-16), Bool(true), 3)),
                        2
                    )),
                    Box::new(EInt(Env::new(), 8, 2)),
                    1
                )),
                Box::new(BPlus(Int(3), Int(8), Int(11), 1)),
                0
            )
        );
    }

    #[test]
    fn test_solve6() {
        assert_eq!(
            solve(
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
                ),
                0
            ),
            EPlus(
                Env::new(),
                Plus(
                    Box::new(Int(3)),
                    Box::new(If(
                        Box::new(Lt(
                            Box::new(Int(-23)),
                            Box::new(Times(Box::new(Int(-2)), Box::new(Int(8))))
                        )),
                        Box::new(Int(8)),
                        Box::new(Int(2))
                    ))
                ),
                Int(4),
                Box::new(EPlus(
                    Env::new(),
                    Int(3),
                    If(
                        Box::new(Lt(
                            Box::new(Int(-23)),
                            Box::new(Times(Box::new(Int(-2)), Box::new(Int(8))))
                        )),
                        Box::new(Int(8)),
                        Box::new(Int(2))
                    ),
                    Box::new(EInt(Env::new(), 3, 2)),
                    Box::new(EIfT(
                        Env::new(),
                        Lt(
                            Box::new(Int(-23)),
                            Box::new(Times(Box::new(Int(-2)), Box::new(Int(8))))
                        ),
                        Int(8),
                        Int(2),
                        Box::new(ELt(
                            Env::new(),
                            Int(-23),
                            Times(Box::new(Int(-2)), Box::new(Int(8))),
                            Box::new(EInt(Env::new(), -23, 4)),
                            Box::new(ETimes(
                                Env::new(),
                                Int(-2),
                                Int(8),
                                Box::new(EInt(Env::new(), -2, 5)),
                                Box::new(EInt(Env::new(), 8, 5)),
                                Box::new(BTimes(Int(-2), Int(8), Int(-16), 5)),
                                4
                            )),
                            Box::new(BLt(Int(-23), Int(-16), Bool(true), 4)),
                            3
                        )),
                        Box::new(EInt(Env::new(), 8, 3)),
                        2
                    )),
                    Box::new(BPlus(Int(3), Int(8), Int(11), 2)),
                    1
                )),
                Box::new(EInt(Env::new(), 4, 1)),
                Box::new(BPlus(Int(11), Int(4), Int(15), 1)),
                0
            )
        );
    }

    #[test]
    fn test_solve7() {
        assert_eq!(
            solve(
                &Env::from(vec![
                    (Var("x".to_string()), Int(3)),
                    (Var("y".to_string()), Int(2))
                ]),
                &Var("x".to_string()),
                0
            ),
            EVar2(
                Env::from(vec![
                    (Var("x".to_string()), Int(3)),
                    (Var("y".to_string()), Int(2))
                ]),
                Var("x".to_string()),
                Box::new(EVar1(
                    Env::from(vec![(Var("x".to_string()), Int(3))]),
                    Var("x".to_string()),
                    1
                )),
                0
            )
        );
    }

    #[test]
    fn test_solve8() {
        assert_eq!(
            solve(
                &Env::from(vec![
                    (Var("x".to_string()), Bool(true)),
                    (Var("y".to_string()), Int(4))
                ]),
                &If(
                    Box::new(Var("x".to_string())),
                    Box::new(Plus(Box::new(Var("y".to_string())), Box::new(Int(1)))),
                    Box::new(Var("y".to_string()))
                ),
                0
            ),
            EIfT(
                Env::from(vec![
                    (Var("x".to_string()), Bool(true)),
                    (Var("y".to_string()), Int(4))
                ]),
                Var("x".to_string()),
                Plus(Box::new(Var("y".to_string())), Box::new(Int(1))),
                Var("y".to_string()),
                Box::new(EVar2(
                    Env::from(vec![
                        (Var("x".to_string()), Bool(true)),
                        (Var("y".to_string()), Int(4))
                    ]),
                    Var("x".to_string()),
                    Box::new(EVar1(
                        Env::from(vec![(Var("x".to_string()), Bool(true))]),
                        Var("x".to_string()),
                        2
                    )),
                    1
                )),
                Box::new(EPlus(
                    Env::from(vec![
                        (Var("x".to_string()), Bool(true)),
                        (Var("y".to_string()), Int(4))
                    ]),
                    Var("y".to_string()),
                    Int(1),
                    Box::new(EVar1(
                        Env::from(vec![
                            (Var("x".to_string()), Bool(true)),
                            (Var("y".to_string()), Int(4))
                        ]),
                        Var("y".to_string()),
                        2
                    )),
                    Box::new(EInt(
                        Env::from(vec![
                            (Var("x".to_string()), Bool(true)),
                            (Var("y".to_string()), Int(4))
                        ]),
                        1,
                        2
                    )),
                    Box::new(BPlus(Int(4), Int(1), Int(5), 2)),
                    1
                )),
                0
            )
        );
    }

    #[test]
    fn test_solve9() {
        assert_eq!(
            solve(
                &Env::new(),
                &Let(
                    Box::new(Var("x".to_string())),
                    Box::new(Plus(Box::new(Int(1)), Box::new(Int(2)))),
                    Box::new(Times(Box::new(Var("x".to_string())), Box::new(Int(4))))
                ),
                0
            ),
            ELet(
                Env::new(),
                Var("x".to_string()),
                Plus(Box::new(Int(1)), Box::new(Int(2))),
                Times(Box::new(Var("x".to_string())), Box::new(Int(4))),
                Box::new(EPlus(
                    Env::new(),
                    Int(1),
                    Int(2),
                    Box::new(EInt(Env::new(), 1, 2)),
                    Box::new(EInt(Env::new(), 2, 2)),
                    Box::new(BPlus(Int(1), Int(2), Int(3), 2)),
                    1
                )),
                Box::new(ETimes(
                    Env::from(vec![(Var("x".to_string()), Int(3))]),
                    Var("x".to_string()),
                    Int(4),
                    Box::new(EVar1(
                        Env::from(vec![(Var("x".to_string()), Int(3))]),
                        Var("x".to_string()),
                        2
                    )),
                    Box::new(EInt(Env::from(vec![(Var("x".to_string()), Int(3))]), 4, 2)),
                    Box::new(BTimes(Int(3), Int(4), Int(12), 2)),
                    1
                )),
                0
            )
        );
    }

    #[test]
    fn test_solve10() {
        assert_eq!(
            solve(
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
                ),
                0
            ),
            ELet(
                Env::new(),
                Var("x".to_string()),
                Times(Box::new(Int(3)), Box::new(Int(3))),
                Let(
                    Box::new(Var("y".to_string())),
                    Box::new(Times(Box::new(Int(4)), Box::new(Var("x".to_string())))),
                    Box::new(Plus(
                        Box::new(Var("x".to_string())),
                        Box::new(Var("y".to_string()))
                    ))
                ),
                Box::new(ETimes(
                    Env::new(),
                    Int(3),
                    Int(3),
                    Box::new(EInt(Env::new(), 3, 2)),
                    Box::new(EInt(Env::new(), 3, 2)),
                    Box::new(BTimes(Int(3), Int(3), Int(9), 2)),
                    1
                )),
                Box::new(ELet(
                    Env::from(vec![(Var("x".to_string()), Int(9))]),
                    Var("y".to_string()),
                    Times(Box::new(Int(4)), Box::new(Var("x".to_string()))),
                    Plus(
                        Box::new(Var("x".to_string())),
                        Box::new(Var("y".to_string()))
                    ),
                    Box::new(ETimes(
                        Env::from(vec![(Var("x".to_string()), Int(9))]),
                        Int(4),
                        Var("x".to_string()),
                        Box::new(EInt(Env::from(vec![(Var("x".to_string()), Int(9))]), 4, 3)),
                        Box::new(EVar1(
                            Env::from(vec![(Var("x".to_string()), Int(9))]),
                            Var("x".to_string()),
                            3
                        )),
                        Box::new(BTimes(Int(4), Int(9), Int(36), 3)),
                        2
                    )),
                    Box::new(EPlus(
                        Env::from(vec![
                            (Var("x".to_string()), Int(9)),
                            (Var("y".to_string()), Int(36))
                        ]),
                        Var("x".to_string()),
                        Var("y".to_string()),
                        Box::new(EVar2(
                            Env::from(vec![
                                (Var("x".to_string()), Int(9)),
                                (Var("y".to_string()), Int(36))
                            ]),
                            Var("x".to_string()),
                            Box::new(EVar1(
                                Env::from(vec![(Var("x".to_string()), Int(9))]),
                                Var("x".to_string()),
                                4
                            )),
                            3
                        )),
                        Box::new(EVar1(
                            Env::from(vec![
                                (Var("x".to_string()), Int(9)),
                                (Var("y".to_string()), Int(36))
                            ]),
                            Var("y".to_string()),
                            3
                        )),
                        Box::new(BPlus(Int(9), Int(36), Int(45), 3)),
                        2
                    )),
                    1
                )),
                0
            )
        );
    }

    #[test]
    fn test_solve11() {
        assert_eq!(
            solve(
                &Env::from(vec![(Var("x".to_string()), Int(3))]),
                &Let(
                    Box::new(Var("x".to_string())),
                    Box::new(Times(Box::new(Var("x".to_string())), Box::new(Int(2)))),
                    Box::new(Plus(
                        Box::new(Var("x".to_string())),
                        Box::new(Var("x".to_string()))
                    )),
                ),
                0
            ),
            ELet(
                Env::from(vec![(Var("x".to_string()), Int(3))]),
                Var("x".to_string()),
                Times(Box::new(Var("x".to_string())), Box::new(Int(2))),
                Plus(
                    Box::new(Var("x".to_string())),
                    Box::new(Var("x".to_string()))
                ),
                Box::new(ETimes(
                    Env::from(vec![(Var("x".to_string()), Int(3))]),
                    Var("x".to_string()),
                    Int(2),
                    Box::new(EVar1(
                        Env::from(vec![(Var("x".to_string()), Int(3))]),
                        Var("x".to_string()),
                        2
                    )),
                    Box::new(EInt(Env::from(vec![(Var("x".to_string()), Int(3))]), 2, 2)),
                    Box::new(BTimes(Int(3), Int(2), Int(6), 2)),
                    1
                )),
                Box::new(EPlus(
                    Env::from(vec![
                        (Var("x".to_string()), Int(3)),
                        (Var("x".to_string()), Int(6))
                    ]),
                    Var("x".to_string()),
                    Var("x".to_string()),
                    Box::new(EVar1(
                        Env::from(vec![
                            (Var("x".to_string()), Int(3)),
                            (Var("x".to_string()), Int(6))
                        ]),
                        Var("x".to_string()),
                        2
                    )),
                    Box::new(EVar1(
                        Env::from(vec![
                            (Var("x".to_string()), Int(3)),
                            (Var("x".to_string()), Int(6))
                        ]),
                        Var("x".to_string()),
                        2
                    )),
                    Box::new(BPlus(Int(6), Int(6), Int(12), 2)),
                    1
                )),
                0
            )
        );
    }
}
