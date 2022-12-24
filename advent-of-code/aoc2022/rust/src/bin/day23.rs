use std::cmp;
use std::collections::HashSet;
use std::fs;
use std::ops::Add;

use counter::Counter;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
struct Elf {
    pos: Point,
    intention: Point,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

const N: Point = Point { x: -1, y: 0 };
const S: Point = Point { x: 1, y: 0 };
const E: Point = Point { x: 0, y: 1 };
const W: Point = Point { x: 0, y: -1 };
const NE: Point = Point { x: -1, y: 1 };
const NW: Point = Point { x: -1, y: -1 };
const SE: Point = Point { x: 1, y: 1 };
const SW: Point = Point { x: 1, y: -1 };

fn main() {
    let input = fs::read_to_string("../input/23.txt").unwrap();

    let mut elves = Vec::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                let p = Point {
                    x: i as i32,
                    y: j as i32,
                };
                elves.push(Elf {
                    pos: p,
                    intention: p,
                });
            }
        }
    }

    let fs: [(fn(Point, &HashSet<Point>) -> bool, Point); 4] = [
        (has_no_northern_neighbors, N),
        (has_no_southern_neighbors, S),
        (has_no_western_neighbors, W),
        (has_no_eastern_neighbors, E),
    ];

    let mut c = 0;

    for r in 1.. {
        let positions: HashSet<Point> = HashSet::from_iter((&elves).iter().map(|e| e.pos));
        let mut neighborless = 0;
        for elf in elves.iter_mut() {
            if has_no_neighbors(elf.pos, &positions) {
                elf.intention = elf.pos;
                neighborless += 1;
            } else {
                for i in c..c + 4 {
                    let (f, dir) = fs[i % 4];
                    if f(elf.pos, &positions) {
                        elf.intention = elf.pos + dir;
                        break;
                    }
                }
            }
        }
        c += 1;
        let intentions = (&elves).iter().map(|e| e.intention).collect::<Counter<_>>();

        for elf in elves.iter_mut() {
            if intentions[&elf.intention] > 1 {
                elf.intention = elf.pos;
            } else {
                elf.pos = elf.intention;
            }
        }
        if r == 10 {
            println!("{}", area_covered(&elves) - (elves.len() as i32));
        }
        if neighborless == elves.len() {
            println!("{}", r);
            break;
        }
    }
}

fn area_covered(elves: &[Elf]) -> i32 {
    let (mut n, mut s, mut e, mut w) = (i32::MAX, i32::MIN, i32::MIN, i32::MAX);
    for elf in elves {
        n = cmp::min(n, elf.pos.x);
        s = cmp::max(s, elf.pos.x);
        e = cmp::max(e, elf.pos.y);
        w = cmp::min(w, elf.pos.y);
    }
    (s - n + 1) * (e - w + 1)
}

fn has_no_neighbors(pos: Point, other: &HashSet<Point>) -> bool {
    _has_no_neighbors(pos, other, &[N, NW, W, SW, S, SE, E, NE])
}

fn has_no_northern_neighbors(pos: Point, other: &HashSet<Point>) -> bool {
    _has_no_neighbors(pos, other, &[N, NW, NE])
}

fn has_no_southern_neighbors(pos: Point, other: &HashSet<Point>) -> bool {
    _has_no_neighbors(pos, other, &[S, SW, SE])
}

fn has_no_eastern_neighbors(pos: Point, other: &HashSet<Point>) -> bool {
    _has_no_neighbors(pos, other, &[E, SE, NE])
}

fn has_no_western_neighbors(pos: Point, other: &HashSet<Point>) -> bool {
    _has_no_neighbors(pos, other, &[W, SW, NW])
}

fn _has_no_neighbors(pos: Point, other: &HashSet<Point>, directions: &[Point]) -> bool {
    for p in directions {
        if other.contains(&(pos + *p)) {
            return false;
        }
    }
    true
}
