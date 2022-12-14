use std::cmp;
use std::fs;

use itertools::Itertools;

#[derive(Copy, Clone)]
enum Cave {
    Air,
    Rock,
    Sand,
}

fn main() {
    let mut cave = [[Cave::Air; 600]; 200];

    let input = fs::read_to_string("../input/14.txt").unwrap();

    for line in input.lines() {
        for (pair1, pair2) in line.split(" -> ").tuple_windows() {
            let (x1, y1) = pair1
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            let (x2, y2) = pair2
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            fill_rock(x1, y1, x2, y2, &mut cave);
        }
    }

    
}

fn fill_rock(x1: usize, y1: usize, x2: usize, y2: usize, cave: &mut [[Cave; 600]; 200]) {
    assert!(x1 == x2 || y1 == y2);

    if x1 == x2 {
        let (y1, y2) = (cmp::min(y1, y2), cmp::max(y1, y2));

        for y in y1..=y2 {
            cave[y][x1] = Cave::Rock;
        }
    } else {
        let (x1, x2) = (cmp::min(x1, x2), cmp::max(x1, x2));
        for x in x1..=x2 {
            cave[y1][x] = Cave::Rock;
        }
    }
}
