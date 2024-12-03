use core::str;
use std::io::BufRead;

pub fn a(mut input: impl BufRead) -> u32 {
    let mut string = Vec::new();
    input.read_to_end(&mut string).unwrap();

    let mut total = 0;

    let mut i = 0;
    while i < string.len() {
        if let Some(((n1, n2), j)) = read_mul_instruction(&string[i..]) {
            eprintln!("mul({n1},{n2}) = {}", n1 * n2);
            total += n1 * n2;
            i += j;
        } else {
            i += 1;
        }
    }
    eprintln!();

    total
}

pub fn b(mut input: impl BufRead) -> u32 {
    let mut string = Vec::new();
    input.read_to_end(&mut string).unwrap();

    let mut total = 0;
    let mut enabled = true;
    for instruction in InstructionReader::read(&string) {
        match instruction {
            Instruction::Do => enabled = true,
            Instruction::Dont => enabled = false,
            Instruction::Mul(n1, n2) => {
                if enabled {
                    total += n1 * n2
                }
            }
        }
    }

    total
}

enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

struct InstructionReader<'a> {
    string: &'a [u8],
    i: usize,
}

impl<'a> InstructionReader<'a> {
    fn read(string: &'a [u8]) -> Self {
        Self { string, i: 0 }
    }
}

impl Iterator for InstructionReader<'_> {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        while self.i < self.string.len() {
            if let Some(((n1, n2), k)) = read_mul_instruction(&self.string[self.i..]) {
                eprintln!("mul({n1},{n2}) = {}", n1 * n2);
                self.i += k;
                return Some(Instruction::Mul(n1, n2));
            } else if read_string(&self.string[self.i..], b"do()") {
                eprintln!("do()");
                self.i += b"do()".len();
                return Some(Instruction::Do);
            } else if read_string(&self.string[self.i..], b"don't()") {
                eprintln!("don't()");
                self.i += b"don't()".len();
                return Some(Instruction::Dont);
            } else {
                self.i += 1;
            }
        }
        eprintln!();
        None
    }
}

fn read_mul_instruction(string: &[u8]) -> Option<((u32, u32), usize)> {
    let mut j = 0;

    if !read_string(&string[j..], b"mul(") {
        return None;
    };
    j += b"mul(".len();
    let (n1, k) = read_number(&string[j..])?;
    j += k;
    if !read_char(&string[j..], b',') {
        return None;
    };
    j += 1;
    let (n2, k) = read_number(&string[j..])?;
    j += k;
    if !read_char(&string[j..], b')') {
        return None;
    };
    j += 1;

    Some(((n1, n2), j))
}

fn read_char(string: &[u8], char: u8) -> bool {
    matches!(string.first(), Some(c) if *c == char)
}

fn read_string(string: &[u8], search: &[u8]) -> bool {
    let mut j = 0;
    while j < search.len() && read_char(&string[j..], search[j]) {
        j += 1;
    }
    j == search.len()
}

fn read_number(string: &[u8]) -> Option<(u32, usize)> {
    let mut j = 0;
    while let Some(digit) = &string.get(j)
        && digit.is_ascii_digit()
    {
        j += 1;
    }
    if j == 0 {
        return None;
    }
    let n = str::from_utf8(&string[..j]).unwrap().parse().unwrap();

    Some((n, j))
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    #[test]
    fn test_1a() {
        let example = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = a(BufReader::new(example.as_bytes()));
        assert_eq!(result, 161);
    }

    #[test]
    fn test_1b() {
        let example = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = b(BufReader::new(example.as_bytes()));
        assert_eq!(result, 48);
    }
}
