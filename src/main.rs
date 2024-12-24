#![feature(let_chains)]
#![feature(linked_list_cursors)]
#![feature(iter_collect_into)]
#![feature(try_blocks)]

use std::env::args;
use std::io::{BufRead, BufReader, stdin};

mod day1;
mod day10;
mod day11;
mod day14;
mod day2;
mod day22;
mod day24;
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
        "11b" => day11::b,
        "14a" => day14::a,
        "14b" => day14::b,
        "22a" => day22::a,
        "24a" => day24::a,
        _ => panic!("Unknown day or part"),
    };

    let result = problem(stdin().lock());
    println!("{result}");
}

pub fn read_str(input: &str) -> impl BufRead {
    BufReader::new(input.as_bytes())
}
