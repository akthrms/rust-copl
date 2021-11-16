use solver::eval_ml1::{parser::parse, solver::solve};

fn main() {
    match parse("3 + (if -23 < -2 * 8 then 8 else 2) + 4 evalto 15") {
        Ok((_, expression)) => solve(expression, 0, true),
        Err(e) => println!("{}", e),
    }
}
