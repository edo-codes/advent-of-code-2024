use std::io::BufRead;

pub fn a(input: impl BufRead) -> u64 {
    input
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .map(|mut secret| {
            for _ in 0..2000 {
                secret = next_secret(secret);
            }
            secret
        })
        .sum()
}

fn next_secret(mut secret: u64) -> u64 {
    secret ^= secret * 64;
    secret %= 16777216;
    secret ^= secret / 32;
    secret %= 16777216;
    secret ^= secret * 2048;
    secret %= 16777216;
    secret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_str;

    #[test]
    fn test_a() {
        let example = "1
10
100
2024";
        assert_eq!(a(read_str(example)), 37327623);
    }
}
