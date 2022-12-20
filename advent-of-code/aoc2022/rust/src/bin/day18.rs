use std::collections::HashSet;
use std::fs;
use std::ops::{Add, Sub};

use itertools::Itertools;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

const X: Point = Point { x: 1, y: 0, z: 0 };
const Y: Point = Point { x: 0, y: 1, z: 0 };
const Z: Point = Point { x: 0, y: 0, z: 1 };

fn main() {
    let input = fs::read_to_string("../input/18.txt").unwrap();

    let mut outer_faces: HashSet<(Point, Point, Point, Point)> = HashSet::new();

    for line in input.lines() {
        let (x, y, z) = line
            .split(',')
            .map(|d| d.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();
        let p = Point { x, y, z };
        let faces = unit_cube_to_faces(p);
        outer_faces = &outer_faces ^ &HashSet::from(faces);
    }
    println!("{}", outer_faces.len());
}

fn unit_cube_to_faces(p: Point) -> [(Point, Point, Point, Point); 6] {
    [
        (p, p + X, p + Y, p + X + Y),
        (p + Z, p + X + Z, p + Y + Z, p + X + Y + Z),
        (p, p + Y, p + Z, p + Y + Z),
        (p + X, p + Y + X, p + Z + X, p + Y + Z + X),
        (p, p + X, p + Z, p + X + Z),
        (p + Y, p + X + Y, p + Z + Y, p + X + Z + Y),
    ]
}
