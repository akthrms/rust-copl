use solver::eval_ml2::{parser::parse, solver::solve};

fn main() {
    match parse("|- let x = let y = 3 - 2 in y * y in let y = 4 in x + y") {
        Ok((_, (env, expr))) => {
            let rule = solve(&env, &expr, 0);
            println!("{}", rule);
        }
        Err(e) => println!("{:?}", e),
    }
}
