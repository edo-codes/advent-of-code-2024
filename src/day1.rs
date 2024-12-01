use std::io::BufRead;

pub fn a(_input: impl BufRead) -> anyhow::Result<u32> {
    todo!();
}

pub fn b(_input: impl BufRead) -> anyhow::Result<u32> {
    todo!();
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
