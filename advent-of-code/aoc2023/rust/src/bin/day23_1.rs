use std::collections::{HashMap, HashSet};

use aoc_utils::*;

fn main() {
    let mut max_path = 0;
    loop {
        let path = part2();
        if path > max_path {
            max_path = path;
            println!("{:?}", max_path);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

fn part2() -> usize {
    let map = read_input_to_char_vectors(23);

    // {
    //     "(x, y), (x1, y1)" => 10,
    // }

    // {
    //     "(x, y)" => [((x1, y1), 10)]
    // }

    let mut graph = HashMap::new();

    let (start_row, start_column) = (0, 1);

    let (finish_row, finish_column) = (map.len() - 1, map[0].len() - 2);

    let mut vertices = vec![((start_row, start_column), Direction::Down)];

    let mut considered = HashSet::new();
    considered.insert(((start_row, start_column), Direction::Down));

    while let Some(((row_x, column_x), direction)) = vertices.pop() {
        let mut row = row_x;
        let mut column = column_x;
        let mut direction = direction;
        let mut len = 1;
        loop {
            match direction {
                Direction::Up => {
                    row -= 1;
                }
                Direction::Down => {
                    row += 1;
                }
                Direction::Left => {
                    column -= 1;
                }
                Direction::Right => {
                    column += 1;
                }
            }

            if row == finish_row && column == finish_column {
                graph.insert(((row_x, column_x), (row, column)), len);
                break;
            }

            let neigbors = find_neigbors(row, column, &map);
            let neigbors = neigbors
                .into_iter()
                .filter(|x| *x != direction.opposite())
                .collect::<Vec<_>>();

            if neigbors.len() > 1 {
                graph.insert(((row_x, column_x), (row, column)), len);
                for neigbor in neigbors {
                    if !considered.contains(&((row, column), neigbor)) {
                        vertices.push(((row, column), neigbor));
                        considered.insert(((row, column), neigbor));
                    }
                }
                break;
            }

            if neigbors.is_empty() {
                // println!("empty at {:?}, {:?}", row + 1, column + 1);
                break;
            }

            direction = neigbors[0];
            len += 1;
        }
    }

    // for ((row1, col1), (row2, col2)) in graph.keys() {
    //     if !graph.contains_key(&((*row2, *col2), (*row1, *col1))) {
    //         println!("{:?} -> {:?}", (*row1, *col1), (*row2, *col2));
    //     }
    // }

    #[allow(clippy::type_complexity)]
    let mut edges: HashMap<(usize, usize), Vec<((usize, usize), usize)>> = HashMap::new();

    for (((row1, col1), (row2, col2)), len) in graph {
        edges
            .entry((row1, col1))
            .or_default()
            .push(((row2, col2), len));
    }

    let mut stack = vec![(vec![(start_row, start_column)], 0)];
    // let mut queue = VecDeque::new();
    // queue.push_back((vec![(start_row, start_column)], 0));

    let mut max_len = 0;

    // let mut max_path = vec![];

    while let Some((mut path, mut len)) = stack.pop() {
        // while let Some((mut path, mut len)) = queue.pop_front() {
        for ((x, y), additional_len) in edges.get(&path[path.len() - 1]).unwrap() {
            if (*x, *y) == (finish_row, finish_column) {
                if max_len < len + additional_len {
                    max_len = len + additional_len;
                    // max_path = path.clone();
                }
                // max_len = max_len.max(len + additional_len);
                // max_path = path.clone();
                continue;
            }
            if path.contains(&(*x, *y)) {
                continue;
            }
            path.push((*x, *y));
            len += additional_len;
            // queue.push_back((path.clone(), len));
            stack.push((path.clone(), len));
        }
    }

    // println!("{:?}", max_len);
    max_len

    // let mut map = map;

    // for (row, col) in max_path {
    //     map[row][col] = 'X';
    // }

    // pretty_print(&map);
} // 6382 too low
  // 6314 bfs
  // 6552 too low
  // 6774 too high
  // 6770 wrong

fn find_neigbors(row: usize, col: usize, map: &[Vec<char>]) -> Vec<Direction> {
    let mut result = Vec::new();

    if row > 0 && map[row - 1][col] != '#' {
        result.push(Direction::Up);
    }
    if row < map.len() - 1 && map[row + 1][col] != '#' {
        result.push(Direction::Down);
    }
    if col > 0 && map[row][col - 1] != '#' {
        result.push(Direction::Left);
    }
    if col < map[0].len() - 1 && map[row][col + 1] != '#' {
        result.push(Direction::Right);
    }

    result
}
