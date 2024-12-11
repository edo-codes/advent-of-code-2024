#![feature(let_chains)]
#![feature(linked_list_cursors)]
use std::env::args;
use std::io::stdin;

mod day1;
mod day10;
mod day11;
mod day2;
mod day3;

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
        "3a" => day3::a,
        "3b" => day3::b,
        "10a" => day10::a,
        "10b" => day10::b,
        "11a" => day11::a,
        _ => panic!("Unknown day or part"),
    };

    let result = problem(stdin().lock());
    println!("{result}");
}
