use std::collections::{HashMap, HashSet};

use aoc2023::*;

fn main() {
    part1();
    // part2();
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Brick {
    x: [u32; 2],
    y: [u32; 2],
    z: [u32; 2],
}

impl Brick {
    fn go_down(&self) -> Self {
        Brick {
            x: self.x,
            y: self.y,
            z: [self.z[0] - 1, self.z[1] - 1],
        }
    }

    fn intersect(&self, other: &Self) -> bool {
        (self.x[0] <= other.x[1] && self.x[1] >= other.x[0])
            && (self.y[0] <= other.y[1] && self.y[1] >= other.y[0])
            && (self.z[0] <= other.z[1] && self.z[1] >= other.z[0])
    }
}

fn part1() {
    let input = read_input_to_lines(221);

    let mut bricks = Vec::new();
    for line in input {
        let (a, b) = line.split_once('~').unwrap();

        let a = a
            .split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let b = b
            .split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let brick = Brick {
            x: [a[0].min(b[0]), a[0].max(b[0])],
            y: [a[1].min(b[1]), a[1].max(b[1])],
            z: [a[2].min(b[2]), a[2].max(b[2])],
        };

        bricks.push(brick);
    }

    bricks.sort_by(|a, b| a.z[0].cmp(&b.z[0]));

    loop {
        let mut next_generation = Vec::new();

        for brick in &bricks {
            if brick.z[0] == 1 {
                next_generation.push(brick.clone());
                continue;
            }
            let fallen_brick = brick.go_down();

            let mut stuck = false;
            for other_fallen_brick in &next_generation {
                if brick.intersect(other_fallen_brick) {
                    next_generation.push(brick.clone());
                    stuck = true;
                    break;
                }
            }
            if !stuck {
                next_generation.push(fallen_brick);
            }
        }

        if next_generation == bricks {
            break;
        }
        bricks = next_generation;
    }

    // println!("{:?}", bricks);

    let mut support: HashMap<Brick, Vec<Brick>> = HashMap::new();

    for brick in &bricks {
        for other_brick in &bricks {
            if brick != other_brick {
                if brick.intersect(&other_brick.go_down()) {
                    support
                        .entry(other_brick.clone())
                        .or_default()
                        .push(brick.clone());
                }
            }
        }
    }

    let mut keystones = HashSet::new();

    for support_bricks in support.values() {
        if support_bricks.len() == 1 {
            keystones.insert(support_bricks[0].clone());
        }
    }

    // println!(
    //     "{:?}",
    //     support.values().map(|x| x.len()).collect::<Vec<_>>()
    // );
    println!("{:?}", support);
    println!("{:?}", bricks.len() - keystones.len());
    println!("{:?}", bricks.len());
}
