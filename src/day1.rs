use std::io::BufRead;
use std::iter::zip;

use anyhow::bail;

pub fn a(input: impl BufRead) -> anyhow::Result<u32> {
    let (mut left, mut right) = parse_input(input)?;
    left.sort_unstable();
    right.sort_unstable();
    let total_distance: u32 = zip(left, right).map(|(l, r)| l.abs_diff(r)).sum();
    Ok(total_distance)
}

pub fn b(input: impl BufRead) -> anyhow::Result<u32> {
    let (mut left, mut right) = parse_input(input)?;
    left.sort_unstable();
    right.sort_unstable();

    let n = left.len();
    assert!(n == right.len());

    let mut sim = 0;
    let (mut i, mut j) = (0, 0);
    while i < n {
        let x = left[i];

        let mut left_occ = 0;
        // Count occurences of current id in left list
        while i < n && left[i] == x {
            left_occ += 1;
            i += 1;
        }

        let mut right_occ = 0;
        // Skip ids in right list that don't occur in left list
        while j < n && right[j] < x {
            j += 1;
        }
        // Count occurrences of current id in right list
        while j < n && right[j] == x {
            right_occ += 1;
            j += 1;
        }

        sim += x * left_occ * right_occ;
    }
    Ok(sim)
}

fn parse_input(input: impl BufRead) -> anyhow::Result<(Vec<u32>, Vec<u32>)> {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = Default::default();
    for line in input.lines() {
        let line = line?;
        let mut line = line.split_ascii_whitespace();
        let Some(l) = line.next() else {
            break;
        };
        let (Some(r), None) = (line.next(), line.next()) else {
            bail!("Each line should contain two numbers")
        };
        let (l, r) = (l.parse()?, r.parse()?);
        left.push(l);
        right.push(r);
    }
    Ok((left, right))
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use indoc::indoc;

    use super::*;

    static EXAMPLE: &str = indoc! {"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    "};

    #[test]
    fn test_1a() {
        let result = a(BufReader::new(EXAMPLE.as_bytes())).unwrap();
        assert_eq!(result, 11);
    }

    #[test]
    fn test_1b() {
        let result = b(BufReader::new(EXAMPLE.as_bytes())).unwrap();
        assert_eq!(result, 31);
    }
}
