use std::collections::HashSet;
use std::fs;

fn part1() {
    let mut visited = HashSet::new();
    let (mut x, mut y) = (0, 0);
    visited.insert((x, y));
    let directions = fs::read_to_string("../input/03.txt").unwrap();
    for d in directions.chars() {
        if d == '>' {
            x += 1;
        } else if d == '<' {
            x -= 1;
        } else if d == '^' {
            y += 1;
        } else if d == 'v' {
            y -= 1;
        }
        visited.insert((x, y));
    }
    println!("{}", visited.len());
}

fn part2() {
    let mut visited = HashSet::new();
    let (mut x, mut y) = (0, 0);
    let (mut a, mut b) = (0, 0);
    let directions = fs::read_to_string("../input/03.txt").unwrap();
    for (i, d) in directions.chars().enumerate() {
        if d == '>' {
            if i % 2 == 0 {
                x += 1;
            } else {
                a += 1;
            }
        } else if d == '<' {
            if i % 2 == 0 {
                x -= 1;
            } else {
                a -= 1;
            }
        } else if d == '^' {
            if i % 2 == 0 {
                y += 1;
            } else {
                b += 1;
            }
        } else if d == 'v' {
            if i % 2 == 0 {
                y -= 1;
            } else {
                b -= 1;
            }
        }
        visited.insert((x, y));
        visited.insert((a, b));
    }
    println!("{}", visited.len());
}

fn main() {
    part1();
    part2();
}