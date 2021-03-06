use solver::eval_ml1::{parser::parse, solver::solve};

fn main() {
    match parse("3 + (if -23 < -2 * 8 then 8 else 2) + 4") {
        Ok((_, expr)) => {
            let rule = solve(&expr, 0);
            println!("{}", rule);
        }
        Err(e) => println!("{:?}", e),
    }
}
