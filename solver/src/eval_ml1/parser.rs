use crate::eval_ml1::ast::Expression;
use crate::util::ws;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{opt, recognize},
    multi::many0,
    sequence::{delimited, tuple},
    IResult,
};

// <expression> ::= <term1> [ <' <term1> ]
// <term1> ::= <term2> [ ('+'|'-') <term2> ]*
// <term2> ::= <factor> [ '*' <factor> ]*
// <factor> ::= <value> | <paren> | <if>
// <value> ::= <integer> | <boolean>
// <integer> ::= 数値
// <boolean> ::= 'true' | 'false'
// <paren> ::= '(' <expression> ')'
// <if> ::= 'if' <expression> 'then' <expression> 'else' <expression>

pub fn parse(input: &str) -> IResult<&str, Expression> {
    parse_expression(input)
}

fn parse_expression(input: &str) -> IResult<&str, Expression> {
    let (input, (left, right)) = tuple((parse_term1, opt(parse_less_than)))(input)?;
    let expression = match right {
        Some(right) => Expression::LessThan(Box::new(left), Box::new(right)),
        None => left,
    };
    Ok((input, expression))
}

fn parse_less_than(input: &str) -> IResult<&str, Expression> {
    let (input, (_, expression)) = tuple((ws(char('<')), parse_term1))(input)?;
    Ok((input, expression))
}

fn parse_term1(input: &str) -> IResult<&str, Expression> {
    let (input, (expression, expressions)) = tuple((parse_term2, parse_add_sub))(input)?;
    let expression =
        expressions
            .iter()
            .fold(expression, |left, (operator, right)| match operator {
                '+' => Expression::Add(Box::new(left), Box::new(right.clone())),
                '-' => Expression::Sub(Box::new(left), Box::new(right.clone())),
                _ => unreachable!(),
            });
    Ok((input, expression))
}

fn parse_add_sub(input: &str) -> IResult<&str, Vec<(char, Expression)>> {
    let parse_add = ws(char('+'));
    let parse_sub = ws(char('-'));
    let parse_operator = alt((parse_add, parse_sub));
    let (input, expressions) = many0(tuple((parse_operator, parse_term2)))(input)?;
    Ok((input, expressions))
}

fn parse_term2(input: &str) -> IResult<&str, Expression> {
    let (input, (expression, expressions)) = tuple((parse_factor, parse_mul))(input)?;
    let expression =
        expressions
            .iter()
            .fold(expression, |left, (operator, right)| match operator {
                '*' => Expression::Mul(Box::new(left), Box::new(right.clone())),
                _ => unreachable!(),
            });
    Ok((input, expression))
}

fn parse_mul(input: &str) -> IResult<&str, Vec<(char, Expression)>> {
    let parse_mul = ws(char('*'));
    let (input, expressions) = many0(tuple((parse_mul, parse_factor)))(input)?;
    Ok((input, expressions))
}

fn parse_factor(input: &str) -> IResult<&str, Expression> {
    let (input, expression) = alt((parse_value, parse_paren, parse_if))(input)?;
    Ok((input, expression))
}

fn parse_value(input: &str) -> IResult<&str, Expression> {
    let (input, expression) = alt((parse_integer, parse_boolean))(input)?;
    Ok((input, expression))
}

fn parse_integer(input: &str) -> IResult<&str, Expression> {
    let (input, n) = alt((ws(parse_pos_number), ws(parse_neg_number)))(input)?;
    Ok((input, Expression::Integer(n)))
}

fn parse_pos_number(input: &str) -> IResult<&str, i64> {
    let (input, n) = digit1(input)?;
    let n = n.parse::<i64>().unwrap();
    Ok((input, n))
}

fn parse_neg_number(input: &str) -> IResult<&str, i64> {
    let (input, n) = recognize(tuple((char('-'), digit1)))(input)?;
    let n = n.parse::<i64>().unwrap();
    Ok((input, n))
}

fn parse_boolean(input: &str) -> IResult<&str, Expression> {
    let (input, b) = alt((parse_true, parse_false))(input)?;
    Ok((input, Expression::Boolean(b)))
}

fn parse_true(input: &str) -> IResult<&str, bool> {
    let (input, _) = ws(tag("true"))(input)?;
    Ok((input, true))
}

fn parse_false(input: &str) -> IResult<&str, bool> {
    let (input, _) = ws(tag("false"))(input)?;
    Ok((input, false))
}

fn parse_paren(input: &str) -> IResult<&str, Expression> {
    let parse_lparen = ws(char('('));
    let parse_rparen = ws(char(')'));
    let (input, expression) = delimited(parse_lparen, parse_expression, parse_rparen)(input)?;
    Ok((input, expression))
}

fn parse_if(input: &str) -> IResult<&str, Expression> {
    let (input, (_, expression1, _, expression2, _, expression3)) = tuple((
        ws(tag("if")),
        parse_expression,
        ws(tag("then")),
        parse_expression,
        ws(tag("else")),
        parse_expression,
    ))(input)?;
    let expression = Expression::If(
        Box::new(expression1),
        Box::new(expression2),
        Box::new(expression3),
    );
    Ok((input, expression))
}

#[cfg(test)]
mod tests {
    use crate::eval_ml1::{ast::Expression::*, parser::parse};

    #[test]
    fn test_parse1() {
        assert_eq!(
            parse("3 + 5").unwrap().1,
            Add(Box::new(Integer(3)), Box::new(Integer(5)))
        );
    }

    #[test]
    fn test_parse2() {
        assert_eq!(
            parse("8 - 2 - 3").unwrap().1,
            Sub(
                Box::new(Sub(Box::new(Integer(8)), Box::new(Integer(2)))),
                Box::new(Integer(3))
            )
        );
    }

    #[test]
    fn test_parse3() {
        assert_eq!(
            parse("(4 + 5) * (1 - 10)").unwrap().1,
            Mul(
                Box::new(Add(Box::new(Integer(4)), Box::new(Integer(5)))),
                Box::new(Sub(Box::new(Integer(1)), Box::new(Integer(10))))
            )
        );
    }

    #[test]
    fn test_parse4() {
        assert_eq!(
            parse("if 4 < 5 then 2 + 3 else 8 * 8").unwrap().1,
            If(
                Box::new(LessThan(Box::new(Integer(4)), Box::new(Integer(5)))),
                Box::new(Add(Box::new(Integer(2)), Box::new(Integer(3)))),
                Box::new(Mul(Box::new(Integer(8)), Box::new(Integer(8))))
            )
        );
    }

    #[test]
    fn test_parse5() {
        assert_eq!(
            parse("3 + if -23 < -2 * 8 then 8 else 2 + 4").unwrap().1,
            Add(
                Box::new(Integer(3)),
                Box::new(If(
                    Box::new(LessThan(
                        Box::new(Integer(-23)),
                        Box::new(Mul(Box::new(Integer(-2)), Box::new(Integer(8))))
                    )),
                    Box::new(Integer(8)),
                    Box::new(Add(Box::new(Integer(2)), Box::new(Integer(4))))
                ))
            )
        );
    }

    #[test]
    fn test_parse6() {
        assert_eq!(
            parse("3 + (if -23 < -2 * 8 then 8 else 2) + 4").unwrap().1,
            Add(
                Box::new(Add(
                    Box::new(Integer(3)),
                    Box::new(If(
                        Box::new(LessThan(
                            Box::new(Integer(-23)),
                            Box::new(Mul(Box::new(Integer(-2)), Box::new(Integer(8))))
                        )),
                        Box::new(Integer(8)),
                        Box::new(Integer(2))
                    ))
                )),
                Box::new(Integer(4))
            )
        );
    }
}
