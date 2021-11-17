use crate::eval_ml2::{
    ast::{Expression, Expression::*},
    rule::{Rule, Rule::*},
};

pub fn solve(expression: &Expression, depth: usize) -> Rule {
    match expression {
        Int(i) => EInt(i.clone(), depth),
        Bool(b) => EBool(b.clone(), depth),
        If(expression1, expression2, expression3) => {
            let rule1 = solve(expression1, depth + 1);
            match rule1.evaluated() {
                Bool(true) => {
                    let rule2 = solve(expression2, depth + 1);
                    EIfT(
                        *expression1.clone(),
                        *expression2.clone(),
                        *expression3.clone(),
                        Box::new(rule1),
                        Box::new(rule2),
                        depth,
                    )
                }
                Bool(false) => {
                    let rule2 = solve(expression3, depth + 1);
                    EIfF(
                        *expression1.clone(),
                        *expression2.clone(),
                        *expression3.clone(),
                        Box::new(rule1),
                        Box::new(rule2),
                        depth,
                    )
                }
                _ => unreachable!(),
            }
        }
        Plus(expression1, expression2) => {
            let rule1 = solve(expression1, depth + 1);
            let rule2 = solve(expression2, depth + 1);
            let expression3 = match (rule1.evaluated(), rule2.evaluated()) {
                (Int(i1), Int(i2)) => Int(i1 + i2),
                _ => unreachable!(),
            };
            let rule3 = BPlus(rule1.evaluated(), rule2.evaluated(), expression3, depth + 1);
            EPlus(
                *expression1.clone(),
                *expression2.clone(),
                Box::new(rule1),
                Box::new(rule2),
                Box::new(rule3),
                depth,
            )
        }
        Minus(expression1, expression2) => {
            let rule1 = solve(expression1, depth + 1);
            let rule2 = solve(expression2, depth + 1);
            let expression3 = match (rule1.evaluated(), rule2.evaluated()) {
                (Int(i1), Int(i2)) => Int(i1 - i2),
                _ => unreachable!(),
            };
            let rule3 = BMinus(rule1.evaluated(), rule2.evaluated(), expression3, depth + 1);
            EMinus(
                *expression1.clone(),
                *expression2.clone(),
                Box::new(rule1),
                Box::new(rule2),
                Box::new(rule3),
                depth,
            )
        }
        Times(expression1, expression2) => {
            let rule1 = solve(expression1, depth + 1);
            let rule2 = solve(expression2, depth + 1);
            let expression3 = match (rule1.evaluated(), rule2.evaluated()) {
                (Int(i1), Int(i2)) => Int(i1 * i2),
                _ => unreachable!(),
            };
            let rule3 = BTimes(rule1.evaluated(), rule2.evaluated(), expression3, depth + 1);
            ETimes(
                *expression1.clone(),
                *expression2.clone(),
                Box::new(rule1),
                Box::new(rule2),
                Box::new(rule3),
                depth,
            )
        }
        Lt(expression1, expression2) => {
            let rule1 = solve(expression1, depth + 1);
            let rule2 = solve(expression2, depth + 1);
            let expression3 = match (rule1.evaluated(), rule2.evaluated()) {
                (Int(i1), Int(i2)) => Bool(i1 < i2),
                _ => unreachable!(),
            };
            let rule3 = BLt(rule1.evaluated(), rule2.evaluated(), expression3, depth + 1);
            ELt(
                *expression1.clone(),
                *expression2.clone(),
                Box::new(rule1),
                Box::new(rule2),
                Box::new(rule3),
                depth,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::eval_ml2::{ast::Expression::*, rule::Rule::*, solver::solve};

    #[test]
    fn test_solve1() {
        assert_eq!(
            solve(&Plus(Box::new(Int(3)), Box::new(Int(5))), 0),
            EPlus(
                Int(3),
                Int(5),
                Box::new(EInt(3, 1)),
                Box::new(EInt(5, 1)),
                Box::new(BPlus(Int(3), Int(5), Int(8), 1)),
                0
            )
        );
    }

    #[test]
    fn test_solve2() {
        assert_eq!(
            solve(
                &Minus(
                    Box::new(Minus(Box::new(Int(8)), Box::new(Int(2)))),
                    Box::new(Int(3))
                ),
                0
            ),
            EMinus(
                Minus(Box::new(Int(8)), Box::new(Int(2))),
                Int(3),
                Box::new(EMinus(
                    Int(8),
                    Int(2),
                    Box::new(EInt(8, 2)),
                    Box::new(EInt(2, 2)),
                    Box::new(BMinus(Int(8), Int(2), Int(6), 2)),
                    1
                )),
                Box::new(EInt(3, 1)),
                Box::new(BMinus(Int(6), Int(3), Int(3), 1)),
                0
            )
        );
    }

    #[test]
    fn test_solve3() {
        assert_eq!(
            solve(
                &Times(
                    Box::new(Plus(Box::new(Int(4)), Box::new(Int(5)))),
                    Box::new(Minus(Box::new(Int(1)), Box::new(Int(10))))
                ),
                0
            ),
            ETimes(
                Plus(Box::new(Int(4)), Box::new(Int(5))),
                Minus(Box::new(Int(1)), Box::new(Int(10))),
                Box::new(EPlus(
                    Int(4),
                    Int(5),
                    Box::new(EInt(4, 2)),
                    Box::new(EInt(5, 2)),
                    Box::new(BPlus(Int(4), Int(5), Int(9), 2)),
                    1
                )),
                Box::new(EMinus(
                    Int(1),
                    Int(10),
                    Box::new(EInt(1, 2)),
                    Box::new(EInt(10, 2)),
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
                &If(
                    Box::new(Lt(Box::new(Int(4)), Box::new(Int(5)))),
                    Box::new(Plus(Box::new(Int(2)), Box::new(Int(3)))),
                    Box::new(Times(Box::new(Int(8)), Box::new(Int(8))))
                ),
                0
            ),
            EIfT(
                Lt(Box::new(Int(4)), Box::new(Int(5))),
                Plus(Box::new(Int(2)), Box::new(Int(3))),
                Times(Box::new(Int(8)), Box::new(Int(8))),
                Box::new(ELt(
                    Int(4),
                    Int(5),
                    Box::new(EInt(4, 2)),
                    Box::new(EInt(5, 2)),
                    Box::new(BLt(Int(4), Int(5), Bool(true), 2)),
                    1
                )),
                Box::new(EPlus(
                    Int(2),
                    Int(3),
                    Box::new(EInt(2, 2)),
                    Box::new(EInt(3, 2)),
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
                Int(3),
                If(
                    Box::new(Lt(
                        Box::new(Int(-23)),
                        Box::new(Times(Box::new(Int(-2)), Box::new(Int(8))))
                    )),
                    Box::new(Int(8)),
                    Box::new(Plus(Box::new(Int(2)), Box::new(Int(4))))
                ),
                Box::new(EInt(3, 1)),
                Box::new(EIfT(
                    Lt(
                        Box::new(Int(-23)),
                        Box::new(Times(Box::new(Int(-2)), Box::new(Int(8))))
                    ),
                    Int(8),
                    Plus(Box::new(Int(2)), Box::new(Int(4))),
                    Box::new(ELt(
                        Int(-23),
                        Times(Box::new(Int(-2)), Box::new(Int(8))),
                        Box::new(EInt(-23, 3)),
                        Box::new(ETimes(
                            Int(-2),
                            Int(8),
                            Box::new(EInt(-2, 4)),
                            Box::new(EInt(8, 4)),
                            Box::new(BTimes(Int(-2), Int(8), Int(-16), 4)),
                            3
                        )),
                        Box::new(BLt(Int(-23), Int(-16), Bool(true), 3)),
                        2
                    )),
                    Box::new(EInt(8, 2)),
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
                    Int(3),
                    If(
                        Box::new(Lt(
                            Box::new(Int(-23)),
                            Box::new(Times(Box::new(Int(-2)), Box::new(Int(8))))
                        )),
                        Box::new(Int(8)),
                        Box::new(Int(2))
                    ),
                    Box::new(EInt(3, 2)),
                    Box::new(EIfT(
                        Lt(
                            Box::new(Int(-23)),
                            Box::new(Times(Box::new(Int(-2)), Box::new(Int(8))))
                        ),
                        Int(8),
                        Int(2),
                        Box::new(ELt(
                            Int(-23),
                            Times(Box::new(Int(-2)), Box::new(Int(8))),
                            Box::new(EInt(-23, 4)),
                            Box::new(ETimes(
                                Int(-2),
                                Int(8),
                                Box::new(EInt(-2, 5)),
                                Box::new(EInt(8, 5)),
                                Box::new(BTimes(Int(-2), Int(8), Int(-16), 5)),
                                4
                            )),
                            Box::new(BLt(Int(-23), Int(-16), Bool(true), 4)),
                            3
                        )),
                        Box::new(EInt(8, 3)),
                        2
                    )),
                    Box::new(BPlus(Int(3), Int(8), Int(11), 2)),
                    1
                )),
                Box::new(EInt(4, 1)),
                Box::new(BPlus(Int(11), Int(4), Int(15), 1)),
                0
            )
        );
    }
}
