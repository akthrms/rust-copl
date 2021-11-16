use solver::eval_ml1::{parser::parse, solver::solve};

fn main() {
    match parse("") {
        Ok((_, expression)) => solve(expression, 0, true),
        Err(e) => println!("{}", e),
    }
}
