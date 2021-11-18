use crate::{
    eval_ml2::ast::{Environment, Expression, Expression::*},
    util::ws,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, digit1},
    combinator::{opt, recognize},
    multi::many0,
    sequence::{delimited, tuple},
    IResult,
};

// <expression> ::= <term1> [ <' <term1> ]
// <term1> ::= <term2> [ ( '+' | '-' ) <term2> ]*
// <term2> ::= <factor> [ '*' <factor> ]*
// <factor> ::= <value> | <paren> | <if>
// <value> ::= <int> | <bool>
// <int> ::= 数値
// <bool> ::= 'true' | 'false'
// <paren> ::= '(' <expression> ')'
// <if> ::= 'if' <expression> 'then' <expression> 'else' <expression>

pub fn parse(input: &str) -> IResult<&str, (Environment, Expression)> {
    match input.find("|-") {
        Some(i) => {
            let (environment_input, expression_input) = input.split_at(i);
            let (_, environment) = parse_environment(environment_input)?;
            let (input, expression) = parse_expression(&expression_input[2..])?;
            Ok((input, (environment, expression)))
        }
        None => {
            let (input, expression) = parse_expression(input)?;
            Ok((input, (Environment::empty(), expression)))
        }
    }
}

fn parse_environment(input: &str) -> IResult<&str, Environment> {
    let (input, var) = opt(parse_var)(input)?;
    let var = match var {
        Some(var) => var,
        None => return Ok((input, Environment::empty())),
    };
    let (input, vars) = opt(parse_vars)(input)?;
    let vars = match vars {
        Some(mut vars) => {
            vars.insert(0, var);
            vars
        }
        None => vec![var],
    };
    let environment = Environment::new(vars);
    Ok((input, environment))
}

fn parse_var(input: &str) -> IResult<&str, (String, Expression)> {
    let parse_name = ws(alphanumeric1);
    let parse_equal = ws(char('='));
    let (input, (name, _, expression)) = tuple((parse_name, parse_equal, parse_expression))(input)?;
    let var = (name.to_string(), expression);
    Ok((input, var))
}

fn parse_vars(input: &str) -> IResult<&str, Vec<(String, Expression)>> {
    let (input, vars) = many0(tuple((ws(char(',')), parse_var)))(input)?;
    let vars = vars.into_iter().map(|(_, var)| var).collect();
    Ok((input, vars))
}

fn parse_expression(input: &str) -> IResult<&str, Expression> {
    let (input, (expression1, expression2)) = tuple((parse_term1, opt(parse_lt)))(input)?;
    let expression = match expression2 {
        Some(expression2) => Lt(Box::new(expression1), Box::new(expression2)),
        None => expression1,
    };
    Ok((input, expression))
}

fn parse_lt(input: &str) -> IResult<&str, Expression> {
    let (input, (_, expression)) = tuple((ws(char('<')), parse_term1))(input)?;
    Ok((input, expression))
}

fn parse_term1(input: &str) -> IResult<&str, Expression> {
    let (input, (expression, expressions)) = tuple((parse_term2, parse_plus_minus))(input)?;
    let expression =
        expressions.iter().fold(
            expression,
            |expression1, (operator, expression2)| match operator {
                '+' => Plus(Box::new(expression1), Box::new(expression2.clone())),
                '-' => Minus(Box::new(expression1), Box::new(expression2.clone())),
                _ => unreachable!(),
            },
        );
    Ok((input, expression))
}

fn parse_plus_minus(input: &str) -> IResult<&str, Vec<(char, Expression)>> {
    let parse_plus = ws(char('+'));
    let parse_minus = ws(char('-'));
    let parse_operator = alt((parse_plus, parse_minus));
    let (input, expressions) = many0(tuple((parse_operator, parse_term2)))(input)?;
    Ok((input, expressions))
}

fn parse_term2(input: &str) -> IResult<&str, Expression> {
    let (input, (expression, expressions)) = tuple((parse_factor, parse_times))(input)?;
    let expression =
        expressions.iter().fold(
            expression,
            |expression1, (operator, expression2)| match operator {
                '*' => Times(Box::new(expression1), Box::new(expression2.clone())),
                _ => unreachable!(),
            },
        );
    Ok((input, expression))
}

fn parse_times(input: &str) -> IResult<&str, Vec<(char, Expression)>> {
    let parse_times = ws(char('*'));
    let (input, expressions) = many0(tuple((parse_times, parse_factor)))(input)?;
    Ok((input, expressions))
}

fn parse_factor(input: &str) -> IResult<&str, Expression> {
    let (input, expression) = alt((parse_value, parse_paren, parse_if))(input)?;
    Ok((input, expression))
}

fn parse_value(input: &str) -> IResult<&str, Expression> {
    let (input, expression) = alt((parse_int, parse_bool))(input)?;
    Ok((input, expression))
}

fn parse_int(input: &str) -> IResult<&str, Expression> {
    let (input, i) = alt((ws(parse_pos_number), ws(parse_neg_number)))(input)?;
    let expression = Int(i);
    Ok((input, expression))
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

fn parse_bool(input: &str) -> IResult<&str, Expression> {
    let (input, b) = alt((parse_true, parse_false))(input)?;
    let expression = Bool(b);
    Ok((input, expression))
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
    let expression = If(
        Box::new(expression1),
        Box::new(expression2),
        Box::new(expression3),
    );
    Ok((input, expression))
}

#[cfg(test)]
mod tests {
    use crate::eval_ml2::{
        ast::{Environment, Expression::*},
        parser::parse,
    };

    #[test]
    fn test_parse1() {
        assert_eq!(
            parse("3 + 5").unwrap().1,
            (
                Environment::empty(),
                Plus(Box::new(Int(3)), Box::new(Int(5)))
            )
        );
    }

    #[test]
    fn test_parse2() {
        assert_eq!(
            parse("8 - 2 - 3").unwrap().1,
            (
                Environment::empty(),
                Minus(
                    Box::new(Minus(Box::new(Int(8)), Box::new(Int(2)))),
                    Box::new(Int(3))
                )
            )
        );
    }

    #[test]
    fn test_parse3() {
        assert_eq!(
            parse("(4 + 5) * (1 - 10)").unwrap().1,
            (
                Environment::empty(),
                Times(
                    Box::new(Plus(Box::new(Int(4)), Box::new(Int(5)))),
                    Box::new(Minus(Box::new(Int(1)), Box::new(Int(10))))
                )
            )
        );
    }

    #[test]
    fn test_parse4() {
        assert_eq!(
            parse("if 4 < 5 then 2 + 3 else 8 * 8").unwrap().1,
            (
                Environment::empty(),
                If(
                    Box::new(Lt(Box::new(Int(4)), Box::new(Int(5)))),
                    Box::new(Plus(Box::new(Int(2)), Box::new(Int(3)))),
                    Box::new(Times(Box::new(Int(8)), Box::new(Int(8))))
                )
            )
        );
    }

    #[test]
    fn test_parse5() {
        assert_eq!(
            parse("3 + if -23 < -2 * 8 then 8 else 2 + 4").unwrap().1,
            (
                Environment::empty(),
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
            )
        );
    }

    #[test]
    fn test_parse6() {
        assert_eq!(
            parse("3 + (if -23 < -2 * 8 then 8 else 2) + 4").unwrap().1,
            (
                Environment::empty(),
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
            )
        );
    }
}