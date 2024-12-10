use std::io::BufRead;
use std::iter::zip;

// O(n log n)
pub fn a(input: impl BufRead) -> u32 {
    let (mut left, mut right) = parse_input(input);
    left.sort_unstable();
    right.sort_unstable();
    let total_distance: u32 = zip(left, right).map(|(l, r)| l.abs_diff(r)).sum();
    total_distance
}

// O(n log n), brute force would be O(n²)
pub fn b(input: impl BufRead) -> u32 {
    let (mut left, mut right) = parse_input(input);
    left.sort_unstable();
    right.sort_unstable();

    let n = left.len();
    assert!(n == right.len());

    let mut similarity = 0;
    let (mut i, mut j) = (0, 0);
    while i < n {
        let curr_num = left[i];

        let mut left_occ = 0;
        // Count occurences of current id in left list
        while i < n && left[i] == curr_num {
            left_occ += 1;
            i += 1;
        }

        let mut right_occ = 0;
        // Skip ids in right list that don't occur in left list
        while j < n && right[j] < curr_num {
            j += 1;
        }
        // Count occurrences of current id in right list
        while j < n && right[j] == curr_num {
            right_occ += 1;
            j += 1;
        }

        similarity += curr_num * left_occ * right_occ;
    }
    similarity
}

fn parse_input(input: impl BufRead) -> (Vec<u32>, Vec<u32>) {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = Default::default();
    for line in input.lines() {
        let line = line.unwrap();
        let mut line = line.split_ascii_whitespace();
        left.push(line.next().unwrap().parse().unwrap());
        right.push(line.next().unwrap().parse().unwrap());
        assert!(line.next().is_none())
    }
    (left, right)
}

#[cfg(test)]
mod tests {
    use advent_of_code_2024::read_str;

    use super::*;

    static EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn test_a() {
        let result = a(read_str(EXAMPLE));
        assert_eq!(result, 11);
    }

    #[test]
    fn test_b() {
        let result = b(read_str(EXAMPLE));
        assert_eq!(result, 31);
    }
}
