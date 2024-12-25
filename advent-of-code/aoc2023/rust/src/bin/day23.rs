use std::collections::HashSet;

use aoc_utils::*;

fn main() {
    part1();
    part2();
}

#[derive(Debug, Clone)]
struct Path {
    v: Vec<(usize, usize)>,
    s: HashSet<(usize, usize)>,
}

impl Path {
    fn new(v: Vec<(usize, usize)>) -> Self {
        Self {
            s: HashSet::from_iter(v.iter().cloned()),
            v,
        }
    }

    fn contains(&self, v: (usize, usize)) -> bool {
        self.s.contains(&v)
    }

    fn push(&mut self, v: (usize, usize)) {
        self.v.push(v);
        self.s.insert(v);
    }
}

fn part1() {
    let map = read_input_to_char_vectors(23);

    let (start_row, start_column) = (0, 1);

    let (finish_row, finish_column) = (map.len() - 1, map[0].len() - 2);

    let mut pathes = vec![Path::new(vec![
        (start_row, start_column),
        (start_row + 1, start_column),
    ])];
    let mut completed = Vec::new();

    while let Some(path) = pathes.pop() {
        let nexts = next1(&path, &map);
        if nexts.is_empty() {
            continue;
        }
        for next in nexts {
            let mut path = path.clone();
            path.push(next);
            if next.0 == finish_row && next.1 == finish_column {
                completed.push(path);
            } else {
                pathes.push(path);
            }
        }
    }
    println!(
        "{}",
        completed.into_iter().map(|x| x.v.len()).max().unwrap() - 1
    );
}

fn next1(path: &Path, map: &[Vec<char>]) -> Vec<(usize, usize)> {
    let (r, c) = path.v.last().unwrap();

    let result = match map[*r][*c] {
        '>' => vec![(*r, c + 1)],
        '<' => vec![(*r, c - 1)],
        '^' => vec![(r - 1, *c)],
        'v' => vec![(r + 1, *c)],
        _ => {
            let mut result = Vec::new();
            for (row, col) in [(r - 1, *c), (*r, c - 1), (*r, c + 1), (r + 1, *c)] {
                if map[row][col] != '#' {
                    result.push((row, col));
                }
            }
            result
        }
    };

    result.into_iter().filter(|x| !path.contains(*x)).collect()
}

fn part2() {
    let map = read_input_to_char_vectors(23);

    let (start_row, start_column) = (0, 1);

    let (finish_row, finish_column) = (map.len() - 1, map[0].len() - 2);

    let mut pathes = vec![Path::new(vec![
        (start_row, start_column),
        (start_row + 1, start_column),
    ])];
    let mut completed = Vec::new();

    while let Some(path) = pathes.pop() {
        let nexts = next2(&path, &map);
        if nexts.is_empty() {
            continue;
        }
        for next in nexts {
            let mut path = path.clone();
            path.push(next);
            if next.0 == finish_row && next.1 == finish_column {
                completed.push(path);
            } else {
                pathes.push(path);
            }
        }
    }
    println!(
        "{}",
        completed.into_iter().map(|x| x.v.len()).max().unwrap() - 1
    );
}

fn next2(path: &Path, map: &[Vec<char>]) -> Vec<(usize, usize)> {
    let (r, c) = path.v.last().unwrap();

    let mut result = Vec::new();
    for (row, col) in [(r - 1, *c), (*r, c - 1), (*r, c + 1), (r + 1, *c)] {
        if map[row][col] != '#' {
            result.push((row, col));
        }
    }

    result.into_iter().filter(|x| !path.contains(*x)).collect()
}
