use std::env::args;
use std::io::stdin;

mod day1;
mod day2;

fn main() {
    let problem = match args()
        .nth(1)
        .expect("Pass day+part in first argument (e.g. 1a)")
        .as_str()
    {
        "1a" => day1::a,
        "1b" => day1::b,
        "2a" => day2::a,
        "2b" => day2::b,
        _ => panic!("Unknown day"),
    };

    let result = problem(stdin().lock());
    println!("{result}");
}
