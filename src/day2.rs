use std::io::BufRead;

// O(#reports * #levels)
pub fn a(input: impl BufRead) -> u64 {
    input
        .lines()
        .filter(|line| {
            let line = line.as_ref().unwrap();
            let report: Vec<u32> = line
                .split_ascii_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            check_report(&report, None)
        })
        .count() as _
}

// O(#reports * #levels²)
pub fn b(input: impl BufRead) -> u64 {
    input
        .lines()
        .filter(|line| {
            let line = line.as_ref().unwrap();
            let report: Vec<u32> = line
                .split_ascii_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            if check_report(&report, None) {
                return true;
            }
            for i in 0..report.len() {
                if check_report(&report, Some(i)) {
                    return true;
                }
            }
            false
        })
        .count() as _
}

fn check_report(report: &[u32], skip_index: Option<usize>) -> bool {
    let mut report = report.iter().copied();
    let mut i = 0;
    if skip_index == Some(0) {
        report.next();
    }
    let mut l_prev = report.next().unwrap();
    let mut increasing = None;
    for l_curr in report {
        i += 1;
        if Some(i) == skip_index {
            continue;
        }
        let curr_increasing = l_prev < l_curr;
        if !((1..=3).contains(&l_prev.abs_diff(l_curr)))
            || (increasing.is_some() && Some(curr_increasing) != increasing)
        {
            return false;
        }
        l_prev = l_curr;
        increasing = Some(curr_increasing);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_str;

    static EXAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test_a() {
        let input = read_str(EXAMPLE);
        assert_eq!(a(input), 2);
    }

    #[test]
    fn test_b() {
        let input = read_str(EXAMPLE);
        assert_eq!(b(input), 4);
    }

    #[test]
    fn test_b_own() {
        // Safe by removing the second level, 4
        let input = read_str("1 4 3 4");
        assert_eq!(b(input), 1);
    }
}
