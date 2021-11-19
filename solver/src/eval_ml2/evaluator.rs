use crate::eval_ml2::ast::{Expr, Expr::*};

pub fn eval(expr: &Expr) -> Expr {
    match expr {
        Int(i) => Int(i.clone()),
        Bool(b) => Bool(b.clone()),
        If(expr1, expr2, expr3) => match eval(expr1) {
            Bool(true) => eval(expr2),
            Bool(false) => eval(expr3),
            _ => unreachable!(),
        },
        Plus(expr1, expr2) => match (eval(expr1), eval(expr2)) {
            (Int(i1), Int(i2)) => Int(i1 + i2),
            _ => unreachable!(),
        },
        Minus(expr1, expr2) => match (eval(expr1), eval(expr2)) {
            (Int(i1), Int(i2)) => Int(i1 - i2),
            _ => unreachable!(),
        },
        Times(expr1, expr2) => match (eval(expr1), eval(expr2)) {
            (Int(i1), Int(i2)) => Int(i1 * i2),
            _ => unreachable!(),
        },
        Lt(expr1, expr2) => match (eval(expr1), eval(expr2)) {
            (Int(i1), Int(i2)) => Bool(i1 < i2),
            _ => unreachable!(),
        },
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::eval_ml2::{ast::Expr::*, evaluator::eval};

    #[test]
    fn test_eval1() {
        assert_eq!(eval(&Plus(Box::new(Int(3)), Box::new(Int(5)))), Int(8));
    }

    #[test]
    fn test_eval2() {
        assert_eq!(
            eval(&Minus(
                Box::new(Minus(Box::new(Int(8)), Box::new(Int(2)))),
                Box::new(Int(3))
            )),
            Int(3)
        );
    }

    #[test]
    fn test_eval3() {
        assert_eq!(
            eval(&Times(
                Box::new(Plus(Box::new(Int(4)), Box::new(Int(5)))),
                Box::new(Minus(Box::new(Int(1)), Box::new(Int(10))))
            )),
            Int(-81)
        );
    }

    #[test]
    fn test_eval4() {
        assert_eq!(
            eval(&If(
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
            eval(&Plus(
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
            eval(&Plus(
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
