use std::collections::{HashSet, VecDeque};
use std::fs;

fn main() {
    let input = fs::read_to_string("../input/12.txt").unwrap();
    let mut map = Vec::new();
    let (mut s_x, mut s_y) = (0, 0);
    let (mut e_x, mut e_y) = (0, 0);
    for (i, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                (s_x, s_y) = (i, j);
                row.push('a' as u8);
            } else if c == 'E' {
                (e_x, e_y) = (i, j);
                row.push('z' as u8);
            } else {
                row.push(c as u8);
            }
        }
        map.push(row);
    }

    println!("{}", bfs(s_x, s_y, e_x, e_y, &map));

    let mut lengths = Vec::new();

    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == b'a' {
                lengths.push(bfs(i, j, e_x, e_y, &map));
            }
        }
    }

    println!("{}", lengths.iter().min().unwrap())
}

fn bfs(s_x: usize, s_y: usize, e_x: usize, e_y: usize, map: &Vec<Vec<u8>>) -> i32 {
    let mut q = VecDeque::new();
    q.push_back((s_x, s_y, 0));

    let mut visited = HashSet::new();

    while !q.is_empty() {
        let (x, y, steps) = q.pop_front().unwrap();
        if (x, y) == (e_x, e_y) {
            return steps;
        }

        let h = map[x][y];
        if can_go((x as isize) - 1, y as isize, h, &map, &visited) {
            q.push_back((x - 1, y, steps + 1));
            visited.insert((x - 1, y));
        }

        if can_go((x as isize) + 1, y as isize, h, &map, &visited) {
            q.push_back((x + 1, y, steps + 1));
            visited.insert((x + 1, y));
        }

        if can_go(x as isize, (y as isize) - 1, h, &map, &visited) {
            q.push_back((x, y - 1, steps + 1));
            visited.insert((x, y - 1));
        }

        if can_go(x as isize, (y as isize) + 1, h, &map, &visited) {
            q.push_back((x, y + 1, steps + 1));
            visited.insert((x, y + 1));
        }
    }
    return i32::MAX;
}

fn can_go(
    x: isize,
    y: isize,
    h: u8,
    map: &Vec<Vec<u8>>,
    visited: &HashSet<(usize, usize)>,
) -> bool {
    x >= 0
        && (x as usize) < map.len()
        && y >= 0
        && (y as usize) < map[x as usize].len()
        && (h > map[x as usize][y as usize] || map[x as usize][y as usize] - h < 2)
        && !visited.contains(&(x as usize, y as usize))
}
