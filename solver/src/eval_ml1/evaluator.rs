use crate::eval_ml1::ast::{Expression, Expression::*};

pub fn evaluate(expression: Expression) -> Expression {
    match expression {
        Integer(n) => Integer(n),
        Boolean(b) => Boolean(b),
        If(condition, consequence, alternative) => match evaluate(*condition) {
            Boolean(true) => evaluate(*consequence),
            Boolean(false) => evaluate(*alternative),
            _ => unreachable!(),
        },
        Plus(left, right) => match (evaluate(*left), evaluate(*right)) {
            (Integer(n), Integer(m)) => Integer(n + m),
            _ => unreachable!(),
        },
        Minus(left, right) => match (evaluate(*left), evaluate(*right)) {
            (Integer(n), Integer(m)) => Integer(n - m),
            _ => unreachable!(),
        },
        Times(left, right) => match (evaluate(*left), evaluate(*right)) {
            (Integer(n), Integer(m)) => Integer(n * m),
            _ => unreachable!(),
        },
        LessThan(left, right) => match (evaluate(*left), evaluate(*right)) {
            (Integer(n), Integer(m)) => Boolean(n < m),
            _ => unreachable!(),
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::eval_ml1::{ast::Expression::*, evaluator::evaluate, parser::parse};

    #[test]
    fn test_evaluate1() {
        assert_eq!(evaluate(parse("3 + 5").unwrap().1), Integer(8));
    }

    #[test]
    fn test_evaluate2() {
        assert_eq!(evaluate(parse("8 - 2 - 3").unwrap().1), Integer(3));
    }

    #[test]
    fn test_evaluate3() {
        assert_eq!(
            evaluate(parse("(4 + 5) * (1 - 10)").unwrap().1),
            Integer(-81)
        );
    }

    #[test]
    fn test_evaluate4() {
        assert_eq!(
            evaluate(parse("if 4 < 5 then 2 + 3 else 8 * 8").unwrap().1),
            Integer(5)
        );
    }

    #[test]
    fn test_evaluate5() {
        assert_eq!(
            evaluate(parse("3 + if -23 < -2 * 8 then 8 else 2 + 4").unwrap().1),
            Integer(11)
        );
    }

    #[test]
    fn test_evaluate6() {
        assert_eq!(
            evaluate(parse("3 + (if -23 < -2 * 8 then 8 else 2) + 4").unwrap().1),
            Integer(15)
        );
    }
}
