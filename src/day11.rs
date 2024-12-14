use core::str;
use std::collections::HashMap;
use std::io::{BufRead, read_to_string};

pub fn a(input: impl BufRead) -> u64 {
    let stones = parse_input(input);
    stones.into_iter().map(|n| blink(n, 25)).sum()
}

pub fn b(input: impl BufRead) -> u64 {
    let stones = parse_input(input);
    let mut memo = HashMap::new();
    stones
        .into_iter()
        .map(|n| blink_memo(n, 75, &mut memo))
        .sum()
}

fn parse_input(input: impl BufRead) -> Vec<u64> {
    let mut vec = Vec::<u64>::with_capacity(1_000_000_000);
    read_to_string(input)
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| str::parse::<u64>(s).unwrap())
        .collect_into(&mut vec);
    vec
}

fn blink(n: u64, times: u64) -> u64 {
    if times == 0 {
        return 1;
    }

    if n == 0 {
        blink(1, times - 1)
    } else {
        let num_digits = count_digits(n);
        if num_digits % 2 == 0 {
            blink(n / 10u64.pow(num_digits / 2), times - 1)
                + blink(n % 10u64.pow(num_digits / 2), times - 1)
        } else {
            blink(n * 2024, times - 1)
        }
    }
}

fn blink_memo(n: u64, times: u32, memo: &mut HashMap<(u64, u32), u64>) -> u64 {
    if times == 0 {
        return 1;
    }

    if let Some(&n_mem) = memo.get(&(n, times)) {
        return n_mem;
    }
    let r = {
        if n == 0 {
            blink_memo(1, times - 1, memo)
        } else {
            let num_digits = count_digits(n);
            if num_digits % 2 == 0 {
                blink_memo(n / 10u64.pow(num_digits / 2), times - 1, memo)
                    + blink_memo(n % 10u64.pow(num_digits / 2), times - 1, memo)
            } else {
                blink_memo(n * 2024, times - 1, memo)
            }
        }
    };
    memo.insert((n, times), r);
    r
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
    use super::*;
    use crate::read_str;

    const EXAMPLE: &str = "125 17";

    #[test]
    fn test_a() {
        let input = read_str(EXAMPLE);
        assert_eq!(a(input), 55312);
    }

    #[test]
    fn test_b() {
        let input = read_str(EXAMPLE);
        assert_eq!(b(input), 65601038650482);
    }
}
