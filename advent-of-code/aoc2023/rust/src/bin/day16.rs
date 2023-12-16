use std::collections::HashSet;

use aoc2023::*;

fn main() {
    part1();
    part2();
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use Direction::*;

fn part1() {
    let contraption = read_input_to_char_vectors(16);
    println!("{}", calculate(&contraption, 0, 0, Right));
}

fn part2() {
    let contraption = read_input_to_char_vectors(16);
    let mut count = Vec::new();

    for row in 0..contraption.len() {
        count.push(calculate(&contraption, row, 0, Right));
        count.push(calculate(
            &contraption,
            row,
            contraption[row].len() - 1,
            Left,
        ));
    }

    for col in 0..contraption[0].len() {
        count.push(calculate(&contraption, 0, col, Down));
        count.push(calculate(&contraption, contraption.len() - 1, col, Up))
    }

    println!("{}", count.iter().max().unwrap());
}

fn calculate(contraption: &[Vec<char>], row: usize, col: usize, dir: Direction) -> usize {
    let mut beams = vec![(row, col, dir)];

    let mut energized = HashSet::new();
    let mut visited = HashSet::new();

    while let Some((mut row, mut col, mut dir)) = beams.pop() {
        loop {
            if visited.contains(&(row, col, dir)) {
                break;
            }
            visited.insert((row, col, dir));

            energized.insert((row, col));
            match contraption[row][col] {
                '/' => match dir {
                    Up => dir = Right,
                    Down => dir = Left,
                    Left => dir = Down,
                    Right => dir = Up,
                },
                '\\' => match dir {
                    Up => dir = Left,
                    Down => dir = Right,
                    Left => dir = Up,
                    Right => dir = Down,
                },
                '-' => match dir {
                    Up | Down => {
                        if col < contraption[row].len() - 1 {
                            beams.push((row, col + 1, Right));
                        }
                        dir = Left;
                    }
                    _ => (),
                },
                '|' => match dir {
                    Left | Right => {
                        if row < contraption.len() - 1 {
                            beams.push((row + 1, col, Down));
                        }
                        dir = Up;
                    }
                    _ => (),
                },
                _ => (),
            }
            match dir {
                Up => {
                    if row > 0 {
                        row -= 1;
                    } else {
                        break;
                    }
                }
                Down => {
                    if row < contraption.len() - 1 {
                        row += 1;
                    } else {
                        break;
                    }
                }
                Left => {
                    if col > 0 {
                        col -= 1;
                    } else {
                        break;
                    }
                }
                Right => {
                    if col < contraption[row].len() - 1 {
                        col += 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }
    energized.len()
}
