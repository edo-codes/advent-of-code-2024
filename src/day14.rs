use core::str;
use std::io::BufRead;

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

pub fn a(input: impl BufRead) -> u64 {
    a_inner(input, WIDTH, HEIGHT)
}

fn a_inner(input: impl BufRead, width: i32, height: i32) -> u64 {
    let mut robots = parse_input(input);

    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.step(width, height);
        }
    }

    let (mut top_left, mut top_right, mut bottom_left, mut bottom_right) = (0, 0, 0, 0);
    for robot in robots {
        match robot.quadrant(width, height) {
            Some(Quadrant::TopLeft) => top_left += 1,
            Some(Quadrant::TopRight) => top_right += 1,
            Some(Quadrant::BottomLeft) => bottom_left += 1,
            Some(Quadrant::BottomRight) => bottom_right += 1,
            None => {}
        }
    }

    top_left * top_right * bottom_left * bottom_right
}

pub fn b(input: impl BufRead) -> u64 {
    let mut robots = parse_input(input);

    let mut i = 0;
    while robots
        .iter()
        .filter(|robot| robot.is_center(WIDTH, HEIGHT))
        .count()
        <= robots.len() / 2
    {
        for robot in robots.iter_mut() {
            robot.step(WIDTH, HEIGHT);
        }
        i += 1;
    }

    print_area(robots);

    i
}

#[derive(Debug)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Robot {
    fn step(&mut self, width: i32, height: i32) {
        self.position.0 = (self.position.0 + self.velocity.0) % width;
        if self.position.0 < 0 {
            self.position.0 += width;
        }
        self.position.1 = (self.position.1 + self.velocity.1) % height;
        if self.position.1 < 0 {
            self.position.1 += height;
        }
    }

    fn quadrant(&self, width: i32, height: i32) -> Option<Quadrant> {
        assert!(width % 2 == 1 && height % 2 == 1);
        if self.position.0 < width / 2 && self.position.1 < height / 2 {
            Some(Quadrant::TopLeft)
        } else if self.position.0 > width / 2 && self.position.1 < height / 2 {
            Some(Quadrant::TopRight)
        } else if self.position.0 < width / 2 && self.position.1 > height / 2 {
            Some(Quadrant::BottomLeft)
        } else if self.position.0 > width / 2 && self.position.1 > height / 2 {
            Some(Quadrant::BottomRight)
        } else {
            None
        }
    }

    fn is_center(&self, width: i32, height: i32) -> bool {
        (width / 4..width / 4 * 3).contains(&self.position.0)
            && (height / 4..height / 4 * 3).contains(&self.position.1)
    }
}

fn parse_input(input: impl BufRead) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut pv = line.split(' ');

            let p = pv.next().unwrap();
            assert!(&p[..2] == "p=");
            let mut p = p[2..].split(',');
            let px: i32 = p.next().unwrap().parse().unwrap();
            let py: i32 = p.next().unwrap().parse().unwrap();

            let v = pv.next().unwrap();
            assert!(&v[..2] == "v=");
            let mut v = v[2..].split(',');
            let vx: i32 = v.next().unwrap().parse().unwrap();
            let vy: i32 = v.next().unwrap().parse().unwrap();

            Robot {
                position: (px, py),
                velocity: (vx, vy),
            }
        })
        .collect()
}

fn print_area(robots: Vec<Robot>) {
    let mut area = [[b' '; WIDTH as usize]; HEIGHT as usize];
    for robot in robots {
        area[robot.position.1 as usize][robot.position.0 as usize] = b'*';
    }
    for row in area {
        eprintln!("{}", str::from_utf8(&row).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_str;

    const EXAMPLE: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_a() {
        assert_eq!(a_inner(read_str(EXAMPLE), 11, 7), 12);
    }
}
