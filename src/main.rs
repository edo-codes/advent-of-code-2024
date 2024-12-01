use std::env::args;
use std::io::stdin;

use anyhow::{Context, bail};

mod day1;

fn main() -> anyhow::Result<()> {
    let problem = match args()
        .nth(1)
        .context("Pass day+part in first argument (e.g. 1a)")?
        .as_str()
    {
        "1a" => day1::a,
        "1b" => day1::b,
        _ => bail!("Unknown day"),
    };

    let result = problem(stdin().lock())?;
    println!("{result}");
    Ok(())
}
