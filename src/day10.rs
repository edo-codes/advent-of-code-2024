use std::collections::HashSet;
use std::io::BufRead;

use itertools::Itertools;

type TopoMap = Vec<Vec<u8>>;

pub fn a(input: impl BufRead) -> u64 {
    let map = parse_input(input);
    trailheads(&map).map(|(x, y)| score_trail(&map, x, y)).sum()
}

pub fn b(input: impl BufRead) -> u64 {
    let map = parse_input(input);
    trailheads(&map).map(|(x, y)| rate_trail(&map, x, y)).sum()
}

fn parse_input(input: impl BufRead) -> TopoMap {
    input
        .lines()
        .map(|line| {
            line.unwrap()
                .bytes()
                .map(|c| {
                    assert!(c.is_ascii_digit());
                    c - b'0'
                })
                .collect()
        })
        .collect()
}

fn trailheads(map: &TopoMap) -> impl Iterator<Item = (usize, usize)> {
    let height = map.len();
    let width = map.first().map_or(0, Vec::len);
    (0..width)
        .cartesian_product(0..height)
        .filter(|(x, y)| map[*y][*x] == 0)
}

fn score_trail(map: &TopoMap, x: usize, y: usize) -> u64 {
    fn find_summits(map: &TopoMap, x: usize, y: usize, summits: &mut HashSet<(usize, usize)>) {
        let current_height = map[y][x];
        if current_height == 9 {
            summits.insert((x, y));
            return;
        }
        for (x2, y2) in walk_up(map, x, y) {
            find_summits(map, x2, y2, summits);
        }
    }

    let mut summits = HashSet::new();
    find_summits(map, x, y, &mut summits);
    summits.len() as _
}

fn rate_trail(map: &TopoMap, x: usize, y: usize) -> u64 {
    let current_height = map[y][x];
    if current_height == 9 {
        return 1;
    }
    walk_up(map, x, y)
        .map(|(x2, y2)| rate_trail(map, x2, y2))
        .sum()
}

fn walk_up(map: &TopoMap, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    let current_height = map[y][x];
    let x = x as i32;
    let y = y as i32;
    let width = map[0].len() as i32;
    let height = map.len() as i32;
    [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)]
        .into_iter()
        .filter(move |(x2, y2)| {
            if !((0..width).contains(x2) && (0..height).contains(y2)) {
                return false;
            }
            let new_height = map[*y2 as usize][*x2 as usize];
            new_height == current_height + 1
        })
        .map(|(x2, y2)| (x2 as usize, y2 as usize))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_str;

    const EXAMPLE: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_a() {
        let input = read_str(EXAMPLE);
        assert_eq!(a(input), 36);
    }

    #[test]
    fn test_b() {
        let input = read_str(EXAMPLE);
        assert_eq!(b(input), 81);
    }
}
