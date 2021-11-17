use crate::eval_ml1::ast::{Expression, Expression::*};

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
    }
}

#[cfg(test)]
mod tests {
    use crate::eval_ml1::{ast::Expression::*, evaluator::eval, parser::parse};

    #[test]
    fn test_eval1() {
        assert_eq!(eval(parse("3 + 5").unwrap().1), Int(8));
    }

    #[test]
    fn test_eval2() {
        assert_eq!(eval(parse("8 - 2 - 3").unwrap().1), Int(3));
    }

    #[test]
    fn test_eval3() {
        assert_eq!(eval(parse("(4 + 5) * (1 - 10)").unwrap().1), Int(-81));
    }

    #[test]
    fn test_eval4() {
        assert_eq!(
            eval(parse("if 4 < 5 then 2 + 3 else 8 * 8").unwrap().1),
            Int(5)
        );
    }

    #[test]
    fn test_eval5() {
        assert_eq!(
            eval(parse("3 + if -23 < -2 * 8 then 8 else 2 + 4").unwrap().1),
            Int(11)
        );
    }

    #[test]
    fn test_eval6() {
        assert_eq!(
            eval(parse("3 + (if -23 < -2 * 8 then 8 else 2) + 4").unwrap().1),
            Int(15)
        );
    }
}
