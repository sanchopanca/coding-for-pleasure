use std::collections::HashSet;

use aoc2023::*;

fn main() {
    part1();
    part2();
    // part2();
}

fn part1() {
    let mut map = read_input_to_char_vectors(21);

    let (mut start_row, mut start_col) = (0, 0);

    for (row, line) in map.iter_mut().enumerate() {
        for (col, c) in line.iter_mut().enumerate() {
            if *c == 'S' {
                start_row = row;
                start_col = col;
                *c = '.';
                break;
            }
        }
    }

    println!("{}", solve((start_row, start_col), 64, &map));
}

fn part2() {
    let mut map = read_input_to_char_vectors(211);

    for (row, line) in map.iter_mut().enumerate() {
        for (col, c) in line.iter_mut().enumerate() {
            if *c == 'S' {
                *c = '.';
                break;
            }
        }
    }

    let length = map.len();
    let half = length / 2;

    // for i in 1..50 {
    //     println!("{} {}", i, solve((half, half), i, &map));
    // }
    // let magic = 26501365;
    let magic = 500;

    assert_eq!((magic - half) % length, 0);
    let n = (magic - half) / length;
    assert!(half % 2 == 1); // this means that corners, edges in the center and the center itself are of the same evenness

    let even = solve((half, half), 200, &map); // 200 is enough
    let odd = solve((half, half), 201, &map);

    // println!("{} {}", even, odd);
    // println!("{}", solve((half, half), length, &map));
    assert!([even, odd].contains(&solve((half, half), length + half, &map))); // this means that everything but the outer edge was covered completely

    let mut result = 0;

    let outer_rim = sqn(n);

    result += solve((0, half), length, &map);
    result += solve((length - 1, half), length, &map);
    result += solve((half, 0), length, &map);
    result += solve((half, length - 1), length, &map);

    let edge_len_without_corners = (outer_rim - 4) / 4;

    result += solve((0, 0), length + half, &map) * edge_len_without_corners;
    result += solve((0, length - 1), length + half, &map) * edge_len_without_corners;
    result += solve((length - 1, 0), length + half, &map) * edge_len_without_corners;
    result += solve((length - 1, length - 1), length + half, &map) * edge_len_without_corners;

    let mut is_odd = true;
    let mut previous_sqn = 0;
    for i in 1..n {
        let sqn = sqn(i);
        let x = sqn - previous_sqn;
        if is_odd {
            result += odd * x
        } else {
            // println!("{}", result);
            result += even * x
        }
        is_odd = !is_odd;
        previous_sqn = sqn;
    }

    println!("{}", result);

    // test();
} // 1116410063851869 too high

fn solve(start: (usize, usize), steps: usize, map: &[Vec<char>]) -> usize {
    // let (start_row, start_col) = (0, half); // 194 7226
    // let (start_row, start_col) = (length - 1, half); // 194 7226
    // let (start_row, start_col) = (half, 0); // 194 7226
    // let (start_row, start_col) = (half, length - 1); // 194 7226

    let mut positions = HashSet::new();
    positions.insert(start);

    for _ in 1..=steps {
        let mut new_positions = HashSet::new();
        for (row, col) in positions {
            if row > 0 && map[row - 1][col] == '.' {
                new_positions.insert((row - 1, col));
            }
            if col > 0 && map[row][col - 1] == '.' {
                new_positions.insert((row, col - 1));
            }
            if row < map.len() - 1 && map[row + 1][col] == '.' {
                new_positions.insert((row + 1, col));
            }
            if col < map[0].len() - 1 && map[row][col + 1] == '.' {
                new_positions.insert((row, col + 1));
            }
        }
        positions = new_positions;
    }
    positions.len()
}

// Centered square number
fn sqn(n: usize) -> usize {
    assert!(n > 0);
    n * n + (n - 1) * (n - 1)
}
