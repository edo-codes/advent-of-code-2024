use core::str;
use std::collections::LinkedList;
use std::io::{BufRead, read_to_string};

pub fn a(input: impl BufRead) -> u32 {
    let mut stones = parse_input(input);
    for _ in 0..25 {
        blink(&mut stones);
    }
    stones.len() as u32
}

fn parse_input(input: impl BufRead) -> LinkedList<u64> {
    read_to_string(input)
        .unwrap()
        .split_ascii_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap()
}

fn blink(stones: &mut LinkedList<u64>) {
    let mut cur = stones.cursor_front_mut();
    while let Some(n) = cur.current() {
        let num_digits = count_digits(*n);
        if num_digits % 2 == 0 {
            let left = *n / 10u64.pow(num_digits / 2);
            let right = *n % 10u64.pow(num_digits / 2);
            *n = left;
            cur.insert_after(right);
            cur.move_next();
        } else if *n == 0 {
            *n = 1;
        } else {
            *n = n.checked_mul(2024).unwrap();
        }
        cur.move_next();
    }
}

fn count_digits(mut n: u64) -> u32 {
    let mut i = 1;
    while n >= 10 {
        i += 1;
        n /= 10;
    }
    i
    // Alternatively:
    // (n as f64).log10() as u32 + 1
}

#[cfg(test)]
mod tests {
    use advent_of_code_2024::read_str;

    use super::*;

    #[test]
    fn test_a() {
        let input = read_str("125 17");
        assert_eq!(a(input), 55312);
    }
}
