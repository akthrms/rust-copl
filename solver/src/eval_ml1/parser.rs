use crate::{
    eval_ml1::ast::{Expr, Expr::*},
    util::ws,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{opt, recognize},
    multi::many0,
    sequence::{delimited, tuple},
    IResult,
};

// <expr> ::= <term1> [ <' <term1> ]
// <term1> ::= <term2> [ ( '+' | '-' ) <term2> ]*
// <term2> ::= <factor> [ '*' <factor> ]*
// <factor> ::= <value> | <paren> | <if>
// <value> ::= <int> | <bool>
// <int> ::= 数値
// <bool> ::= 'true' | 'false'
// <paren> ::= '(' <expr> ')'
// <if> ::= 'if' <expr> 'then' <expr> 'else' <expr>

pub fn parse(input: &str) -> IResult<&str, Expr> {
    parse_expr(input)
}

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    let (input, (expr1, expr2)) = tuple((parse_term1, opt(parse_lt)))(input)?;
    let expr = match expr2 {
        Some(expr2) => Lt(Box::new(expr1), Box::new(expr2)),
        None => expr1,
    };
    Ok((input, expr))
}

fn parse_lt(input: &str) -> IResult<&str, Expr> {
    let (input, (_, expr)) = tuple((ws(char('<')), parse_term1))(input)?;
    Ok((input, expr))
}

fn parse_term1(input: &str) -> IResult<&str, Expr> {
    let (input, (expr, exprs)) = tuple((parse_term2, parse_plus_minus))(input)?;
    let expr = exprs.iter().fold(expr, |expr1, (op, expr2)| match op {
        '+' => Plus(Box::new(expr1), Box::new(expr2.clone())),
        '-' => Minus(Box::new(expr1), Box::new(expr2.clone())),
        _ => unreachable!(),
    });
    Ok((input, expr))
}

fn parse_plus_minus(input: &str) -> IResult<&str, Vec<(char, Expr)>> {
    let parse_plus = ws(char('+'));
    let parse_minus = ws(char('-'));
    let parse_op = alt((parse_plus, parse_minus));
    let (input, exprs) = many0(tuple((parse_op, parse_term2)))(input)?;
    Ok((input, exprs))
}

fn parse_term2(input: &str) -> IResult<&str, Expr> {
    let (input, (expr, exprs)) = tuple((parse_factor, parse_times))(input)?;
    let expr = exprs.iter().fold(expr, |expr1, (op, expr2)| match op {
        '*' => Times(Box::new(expr1), Box::new(expr2.clone())),
        _ => unreachable!(),
    });
    Ok((input, expr))
}

fn parse_times(input: &str) -> IResult<&str, Vec<(char, Expr)>> {
    let parse_times = ws(char('*'));
    let (input, exprs) = many0(tuple((parse_times, parse_factor)))(input)?;
    Ok((input, exprs))
}

fn parse_factor(input: &str) -> IResult<&str, Expr> {
    let (input, expr) = alt((parse_value, parse_paren, parse_if))(input)?;
    Ok((input, expr))
}

fn parse_value(input: &str) -> IResult<&str, Expr> {
    let (input, expr) = alt((parse_int, parse_bool))(input)?;
    Ok((input, expr))
}

fn parse_int(input: &str) -> IResult<&str, Expr> {
    let (input, i) = alt((ws(parse_pos_number), ws(parse_neg_number)))(input)?;
    let expr = Int(i);
    Ok((input, expr))
}

fn parse_pos_number(input: &str) -> IResult<&str, i64> {
    let (input, i) = digit1(input)?;
    let i = i.parse::<i64>().unwrap();
    Ok((input, i))
}

fn parse_neg_number(input: &str) -> IResult<&str, i64> {
    let (input, i) = recognize(tuple((char('-'), digit1)))(input)?;
    let i = i.parse::<i64>().unwrap();
    Ok((input, i))
}

fn parse_bool(input: &str) -> IResult<&str, Expr> {
    let (input, b) = alt((parse_true, parse_false))(input)?;
    let expr = Bool(b);
    Ok((input, expr))
}

fn parse_true(input: &str) -> IResult<&str, bool> {
    let (input, _) = ws(tag("true"))(input)?;
    Ok((input, true))
}

fn parse_false(input: &str) -> IResult<&str, bool> {
    let (input, _) = ws(tag("false"))(input)?;
    Ok((input, false))
}

fn parse_paren(input: &str) -> IResult<&str, Expr> {
    let parse_lparen = ws(char('('));
    let parse_rparen = ws(char(')'));
    let (input, expr) = delimited(parse_lparen, parse_expr, parse_rparen)(input)?;
    Ok((input, expr))
}

fn parse_if(input: &str) -> IResult<&str, Expr> {
    let (input, (_, expr1, _, expr2, _, expr3)) = tuple((
        ws(tag("if")),
        parse_expr,
        ws(tag("then")),
        parse_expr,
        ws(tag("else")),
        parse_expr,
    ))(input)?;
    let expr = If(Box::new(expr1), Box::new(expr2), Box::new(expr3));
    Ok((input, expr))
}

#[cfg(test)]
mod tests {
    use crate::eval_ml1::{ast::Expr::*, parser::parse};

    #[test]
    fn test_parse1() {
        assert_eq!(
            parse("3 + 5").unwrap().1,
            Plus(Box::new(Int(3)), Box::new(Int(5)))
        );
    }

    #[test]
    fn test_parse2() {
        assert_eq!(
            parse("8 - 2 - 3").unwrap().1,
            Minus(
                Box::new(Minus(Box::new(Int(8)), Box::new(Int(2)))),
                Box::new(Int(3))
            )
        );
    }

    #[test]
    fn test_parse3() {
        assert_eq!(
            parse("(4 + 5) * (1 - 10)").unwrap().1,
            Times(
                Box::new(Plus(Box::new(Int(4)), Box::new(Int(5)))),
                Box::new(Minus(Box::new(Int(1)), Box::new(Int(10))))
            )
        );
    }

    #[test]
    fn test_parse4() {
        assert_eq!(
            parse("if 4 < 5 then 2 + 3 else 8 * 8").unwrap().1,
            If(
                Box::new(Lt(Box::new(Int(4)), Box::new(Int(5)))),
                Box::new(Plus(Box::new(Int(2)), Box::new(Int(3)))),
                Box::new(Times(Box::new(Int(8)), Box::new(Int(8))))
            )
        );
    }

    #[test]
    fn test_parse5() {
        assert_eq!(
            parse("3 + if -23 < -2 * 8 then 8 else 2 + 4").unwrap().1,
            Plus(
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
        );
    }

    #[test]
    fn test_parse6() {
        assert_eq!(
            parse("3 + (if -23 < -2 * 8 then 8 else 2) + 4").unwrap().1,
            Plus(
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
        );
    }
}
