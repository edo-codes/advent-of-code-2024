use std::io::{BufRead, BufReader};

pub fn read_str(input: &str) -> impl BufRead {
    BufReader::new(input.as_bytes())
}
