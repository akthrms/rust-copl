use crate::eval_ml2::ast::{Expression, Expression::*};

pub fn eval(expression: Expression) -> Expression {
    match expression {
        Int(i) => Int(i),
        Bool(b) => Bool(b),
        If(expression1, expression2, expression3) => match eval(*expression1) {
            Bool(true) => eval(*expression2),
            Bool(false) => eval(*expression3),
            _ => unreachable!(),
        },
        Plus(expression1, expression2) => match (eval(*expression1), eval(*expression2)) {
            (Int(i1), Int(i2)) => Int(i1 + i2),
            _ => unreachable!(),
        },
        Minus(expression1, expression2) => match (eval(*expression1), eval(*expression2)) {
            (Int(i1), Int(i2)) => Int(i1 - i2),
            _ => unreachable!(),
        },
        Times(expression1, expression2) => match (eval(*expression1), eval(*expression2)) {
            (Int(i1), Int(i2)) => Int(i1 * i2),
            _ => unreachable!(),
        },
        Lt(expression1, expression2) => match (eval(*expression1), eval(*expression2)) {
            (Int(i1), Int(i2)) => Bool(i1 < i2),
            _ => unreachable!(),
        },
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::eval_ml2::{ast::Expression::*, evaluator::eval};

    #[test]
    fn test_eval1() {
        assert_eq!(eval(Plus(Box::new(Int(3)), Box::new(Int(5)))), Int(8));
    }

    #[test]
    fn test_eval2() {
        assert_eq!(
            eval(Minus(
                Box::new(Minus(Box::new(Int(8)), Box::new(Int(2)))),
                Box::new(Int(3))
            )),
            Int(3)
        );
    }

    #[test]
    fn test_eval3() {
        assert_eq!(
            eval(Times(
                Box::new(Plus(Box::new(Int(4)), Box::new(Int(5)))),
                Box::new(Minus(Box::new(Int(1)), Box::new(Int(10))))
            )),
            Int(-81)
        );
    }

    #[test]
    fn test_eval4() {
        assert_eq!(
            eval(If(
                Box::new(Lt(Box::new(Int(4)), Box::new(Int(5)))),
                Box::new(Plus(Box::new(Int(2)), Box::new(Int(3)))),
                Box::new(Times(Box::new(Int(8)), Box::new(Int(8))))
            )),
            Int(5)
        );
    }

    #[test]
    fn test_eval5() {
        assert_eq!(
            eval(Plus(
                Box::new(Int(3)),
                Box::new(If(
                    Box::new(Lt(
                        Box::new(Int(-23)),
                        Box::new(Times(Box::new(Int(-2)), Box::new(Int(8))))
                    )),
                    Box::new(Int(8)),
                    Box::new(Plus(Box::new(Int(2)), Box::new(Int(4))))
                ))
            )),
            Int(11)
        );
    }

    #[test]
    fn test_eval6() {
        assert_eq!(
            eval(Plus(
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
            )),
            Int(15)
        );
    }
}
