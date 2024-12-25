use std::{collections::HashMap, ops::RangeInclusive};

use aoc_utils::*;

fn main() {
    solve(parse_part1);
    solve(parse_part2);
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum HorizontalSegment {
    Line(RangeInclusive<i64>),
    Point(i64),
}

impl PartialOrd for HorizontalSegment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// this only works if the segements don't overlap. we make sure they don't just before sorting
impl Ord for HorizontalSegment {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (HorizontalSegment::Line(l1), HorizontalSegment::Line(l2)) => {
                l1.start().cmp(l2.start())
            }
            (HorizontalSegment::Line(l), HorizontalSegment::Point(p)) => l.start().cmp(p),
            (HorizontalSegment::Point(p), HorizontalSegment::Line(l)) => p.cmp(l.start()),
            (HorizontalSegment::Point(p1), HorizontalSegment::Point(p2)) => p1.cmp(p2),
        }
    }
}

enum Dir {
    R,
    D,
    L,
    U,
}

fn solve(parser: fn(&str) -> (i64, Dir)) {
    let input = read_input_to_lines(aoc(18));

    let (mut row, mut col) = (0i64, 0i64);
    let mut row_to_cols = HashMap::new();
    row_to_cols.insert(row, vec![HorizontalSegment::Point(col)]);
    for line in input {
        let (steps, dir) = parser(&line);

        match dir {
            Dir::U => {
                for _ in 0..steps {
                    row -= 1;
                    row_to_cols
                        .entry(row)
                        .or_default()
                        .push(HorizontalSegment::Point(col));
                }
            }
            Dir::D => {
                for _ in 0..steps {
                    row += 1;
                    row_to_cols
                        .entry(row)
                        .or_default()
                        .push(HorizontalSegment::Point(col));
                }
            }
            Dir::R => {
                row_to_cols
                    .entry(row)
                    .or_default()
                    .push(HorizontalSegment::Line(col..=(col + steps)));
                col += steps;
            }
            Dir::L => {
                row_to_cols
                    .entry(row)
                    .or_default()
                    .push(HorizontalSegment::Line((col - steps)..=col));
                col -= steps;
            }
        }
    }

    for row in row_to_cols.values_mut() {
        *row = fix_row(row);
    }

    let mut area = 0;
    for (i, row) in &row_to_cols {
        let mut inside = false;
        let mut previous = 0; // doesn't matter for now, since we are outside
        let mut row_area = 0;
        for segment in row {
            match segment {
                HorizontalSegment::Point(p) => {
                    if inside {
                        row_area += p - previous;
                    } else {
                        row_area += 1;
                    }
                    inside = !inside;
                    previous = *p;
                }
                HorizontalSegment::Line(l) => {
                    row_area += l.end() - l.start() + 1;

                    if inside {
                        row_area += l.start() - previous - 1;
                    }

                    let s_like = if row_to_cols.contains_key(&(i - 1))
                        && row_to_cols.contains_key(&(i + 1))
                    {
                        let bottom_row = row_to_cols.get(&(i + 1)).unwrap();
                        let top_row = row_to_cols.get(&(i - 1)).unwrap();
                        (contains(bottom_row, *l.start()) && contains(top_row, *l.end()))
                            || (contains(top_row, *l.start()) && contains(bottom_row, *l.end()))
                    } else {
                        false
                    };

                    if s_like {
                        inside = !inside;
                    }

                    previous = *l.end();
                }
            }
        }
        area += row_area;
    }

    println!("{}", area);
}

fn parse_part1(line: &str) -> (i64, Dir) {
    let (dir, rest) = line.split_once(' ').unwrap();
    let (steps, _) = rest.split_once(' ').unwrap();
    let steps = steps.parse().unwrap();

    let dir = match dir {
        "U" => Dir::U,
        "D" => Dir::D,
        "L" => Dir::L,
        "R" => Dir::R,
        _ => unreachable!(),
    };
    (steps, dir)
}

fn parse_part2(line: &str) -> (i64, Dir) {
    let (_, rest) = line.split_once('#').unwrap();
    let steps = i64::from_str_radix(&rest.chars().take(5).collect::<String>(), 16).unwrap();

    let dir = match &rest[5..] {
        "0)" => Dir::R,
        "1)" => Dir::D,
        "2)" => Dir::L,
        "3)" => Dir::U,
        _ => unreachable!(),
    };
    (steps, dir)
}

fn fix_row(row: &[HorizontalSegment]) -> Vec<HorizontalSegment> {
    let ranges = row
        .iter()
        .filter_map(|x| match x {
            HorizontalSegment::Line(range) => Some(range.clone()),
            HorizontalSegment::Point(_) => None,
        })
        .collect::<Vec<_>>();
    let numbers = row
        .iter()
        .filter_map(|x| match x {
            HorizontalSegment::Line(_) => None,
            HorizontalSegment::Point(x) => Some(*x),
        })
        .collect::<Vec<_>>();
    let mut result = Vec::new();

    'numbers_loop: for n in numbers {
        for range in &ranges {
            if range.contains(&n) {
                continue 'numbers_loop;
            }
        }
        result.push(HorizontalSegment::Point(n));
    }

    for range in ranges {
        result.push(HorizontalSegment::Line(range));
    }

    result.sort();
    result
}

fn contains(row: &[HorizontalSegment], n: i64) -> bool {
    row.iter().any(|x| match x {
        HorizontalSegment::Line(range) => range.contains(&n),
        HorizontalSegment::Point(x) => *x == n,
    })
}

#[allow(dead_code)]
fn part1_bucket_fill() {
    let input = read_input_to_lines(aoc(18));

    let (mut row, mut col) = (0i32, 0i32);
    let (mut min_row, mut max_row, mut min_col, mut max_col) = (0, 0, 0, 0);
    let mut row_to_cols = HashMap::new();
    row_to_cols.insert(row, vec![col]);
    for line in input {
        let (dir, rest) = line.split_once(' ').unwrap();
        let (steps, _) = rest.split_once(' ').unwrap();
        let steps = steps.parse::<i32>().unwrap();

        for _ in 0..steps {
            match dir {
                "U" => row -= 1,
                "D" => row += 1,
                "L" => col -= 1,
                "R" => col += 1,
                _ => unreachable!(),
            }
            row_to_cols.entry(row).or_default().push(col);
        }
        min_row = min_row.min(row);
        max_row = max_row.max(row);
        min_col = min_col.min(col);
        max_col = max_col.max(col);
    }

    let width = max_col - min_col + 1;
    let height = max_row - min_row + 1;

    let mut map = vec![vec!['.'; width as usize]; height as usize];

    for (row, cols) in row_to_cols {
        for col in cols {
            map[(row - min_row) as usize][(col - min_col) as usize] = '#';
        }
    }

    let left_upper_corner = map.first().unwrap().iter().position(|x| *x == '#').unwrap();

    let mut to_color = vec![(1, left_upper_corner + 1)];

    while let Some((row, col)) = to_color.pop() {
        if map[row][col + 1] == '.' {
            to_color.push((row, col + 1));
        }
        if map[row][col - 1] == '.' {
            to_color.push((row, col - 1));
        }
        if map[row + 1][col] == '.' {
            to_color.push((row + 1, col));
        }
        if map[row - 1][col] == '.' {
            to_color.push((row - 1, col));
        }
        map[row][col] = '#';
    }

    let area = map
        .iter()
        .map(|row| row.iter().filter(|x| **x == '#').count())
        .sum::<usize>();

    println!("{}", area);
}
