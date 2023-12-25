use std::vec;

use aoc2023::*;

fn main() {
    part1();
    // part2();
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// impl From<Direction> for usize {
//     fn from(value: Direction) -> Self {
//         value as Self
//     }
// }

use Direction::*;

const M: usize = 3; // max number of steps at the same direction
const D: usize = 4; // number of directions

fn part1() {
    let input = read_input_to_char_vectors(99);
    let mut weigts = Vec::new();
    for row in input {
        weigts.push(
            row.iter()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<_>>(),
        );
    }
    weigts[0][0] = 0;

    let width = weigts[0].len();
    let height = weigts.len();

    // This is several hundreds of megabytes on real input
    // let mut g = vec![vec![None; width * height]; width * height];
    // for row in 0..weigts.len() {
    //     for col in 0..weigts[0].len() {
    //         let (mut sum_up, mut sum_down, mut sum_left, mut sum_right) = (0, 0, 0, 0);
    //         for i in 1..=3 {
    //             if col + i < width {
    //                 g[row * width + col][row * width + col + i] =
    //                     Some(sum_right + weigts[row][col + i]);
    //                 sum_right += weigts[row][col + i];
    //             }
    //             if col >= i {
    //                 g[row * width + col][row * width + col - i] =
    //                     Some(sum_left + weigts[row][col - i]);
    //                 sum_left += weigts[row][col - i];
    //             }
    //             if row + i < height {
    //                 g[row * width + col][((row + i) * width) + col] =
    //                     Some(sum_down + weigts[row + i][col]);
    //                 sum_down += weigts[row + i][col];
    //             }
    //             if row >= i {
    //                 g[row * width + col][((row - i) * width) + col] =
    //                     Some(sum_up + weigts[row - i][col]);
    //                 sum_up += weigts[row - i][col];
    //             }
    //         }
    //     }
    // }

    // println!("{:?}", g);

    // row, col, direction, number of steps
    let mut distances = vec![vec![vec![vec![u32::MAX; M]; D]; width]; height];
    distances[0][0][Down as usize] = vec![0; M];
    distances[0][0][Right as usize] = vec![0; M];

    let mut visited = vec![vec![vec![vec![false; M]; D]; width]; height];

    for step in 0..M {
        for col in 0..width {
            for i in step..M {
                visited[step][col][Down as usize][i] = true;
                visited[height - 1 - step][col][Up as usize][i] = true;
            }
        }

        for row in 0..height {
            for i in step..M {
                visited[row][step][Right as usize][i] = true;
                visited[row][width - 1 - step][Left as usize][i] = true;
            }
        }
    }

    // assert_eq!(visited[0][0][Down as usize], vec![true; M]);
    // assert_eq!(visited[1][0][Down as usize], vec![false, true, true]);
    // assert_eq!(visited[2][0][Down as usize], vec![false, false, true]);
    // assert_eq!(visited[3][0][Down as usize], vec![false, false, false]);

    // assert_eq!(visited[height - 1][0][Up as usize], vec![true; M]);
    // assert_eq!(visited[height - 2][0][Up as usize], vec![false, true, true]);
    // assert_eq!(
    //     visited[height - 3][0][Up as usize],
    //     vec![false, false, true]
    // );
    // assert_eq!(
    //     visited[height - 4][0][Up as usize],
    //     vec![false, false, false]
    // );

    // assert_eq!(visited[0][0][Right as usize], vec![true; M]);
    // assert_eq!(visited[0][1][Right as usize], vec![false, true, true]);
    // assert_eq!(visited[0][2][Right as usize], vec![false, false, true]);
    // assert_eq!(visited[0][3][Right as usize], vec![false, false, false]);

    // assert_eq!(visited[0][width - 1][Left as usize], vec![true; M]);
    // assert_eq!(
    //     visited[0][width - 2][Left as usize],
    //     vec![false, true, true]
    // );
    // assert_eq!(
    //     visited[0][width - 3][Left as usize],
    //     vec![false, false, true]
    // );
    // assert_eq!(
    //     visited[0][width - 4][Left as usize],
    //     vec![false, false, false]
    // );

    let (mut row, mut col, mut dir_from, mut steps) = (0, 0, Left, 0);

    // TODO: It's not possible to calculate all directions at the same time
    while !last_cell_visited_from_all_directions(&visited) {
        println!("{} {} {:?} {}", row, col, dir_from, steps);
        println!("visited: {:?}", visited[0][0]);
        println!("distances: {:?}", distances[0][0]);
        println!("---------------");
        let dirs_from = [Right, Left, Up, Down];
        // for &dir_from in &dirs_from {
        let rest_dirs = dirs_from
            .iter()
            .filter(|&&x| x != dir_from)
            .map(|&x| x as usize);
        let (r, c) = match dir_from {
            Right => {
                if col == 0 {
                    continue;
                }
                (row, col - 1)
            }
            Left => {
                if col == width - 1 {
                    continue;
                }
                (row, col + 1)
            }
            Up => {
                if row == height - 1 {
                    continue;
                }
                (row + 1, col)
            }
            Down => {
                if row == 0 {
                    continue;
                }
                (row - 1, col)
            }
        };
        let d = dir_from as usize;

        if steps > 0 {
            if !visited[r][c][d][steps] {
                distances[r][c][d][steps] =
                    distances[r][c][d][steps].min(distances[row][col][d][M - steps] + weigts[r][c]);
            }
        } else if !visited[r][c][d][0] {
            for dir in rest_dirs {
                for i in 0..M {
                    distances[r][c][d][0] = distances[r][c][d][0]
                        .min(distances[row][col][dir][i].saturating_add(weigts[r][c]));
                }
            }
        }

        visited[row][col][d][steps] = true;

        (row, col, dir_from, steps) = next(&visited, &distances);
        // return;
        // }
    }

    // while !visited.last().unwrap().last().unwrap() {

    // if row > 0 && !visited[row - 1][col] {
    //     distances[row - 1][col] =
    //         distances[row - 1][col].min(distances[row][col] + weigts[row - 1][col]);
    // }
    // if row < weigts.len() - 1 && !visited[row + 1][col] {
    //     distances[row + 1][col] =
    //         distances[row + 1][col].min(distances[row][col] + weigts[row + 1][col]);
    // }
    // if col > 0 && !visited[row][col - 1] {
    //     distances[row][col - 1] =
    //         distances[row][col - 1].min(distances[row][col] + weigts[row][col - 1]);
    // }
    // if col < weigts[0].len() - 1 && !visited[row][col + 1] {
    //     distances[row][col + 1] =
    //         distances[row][col + 1].min(distances[row][col] + weigts[row][col + 1]);
    // }
    // visited[row][col] = true;
    // (row, col) = next(&visited, &distances);
    // }

    println!("{:?}", distances.last().unwrap().last().unwrap());
}

fn last_cell_visited_from_all_directions(visited: &[Vec<Vec<Vec<bool>>>]) -> bool {
    let last_cell = visited.last().unwrap().last().unwrap();
    // last_cell can only be reached by going down or right
    last_cell[Down as usize].iter().all(|&v| v) && last_cell[Right as usize].iter().all(|&v| v)
}

// fn next_old(visited: &[Vec<bool>], distances: &[Vec<u32>]) -> (usize, usize) {
//     let (mut min_row, mut min_col) = (0, 0);
//     let mut min_distance = u32::MAX;
//     for (r, row) in distances.iter().enumerate() {
//         for (c, &distance) in row.iter().enumerate() {
//             if !visited[r][c] && distance < min_distance {
//                 min_row = r;
//                 min_col = c;
//                 min_distance = distance;
//             }
//         }
//     }
//     (min_row, min_col)
// }

fn next(
    visited: &[Vec<Vec<Vec<bool>>>],
    distances: &[Vec<Vec<Vec<u32>>>],
) -> (usize, usize, Direction, usize) {
    println!("visited: {:?}", visited[0][0]);
    println!("distances: {:?}", distances[0][0]);
    let (mut min_row, mut min_col, mut min_dir, mut min_steps) = (0, 0, Up, 0);
    let mut min_distance = u32::MAX;
    for (r, row) in distances.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            for d in [Up, Down, Left, Right] {
                let steps = &col[d as usize];
                for (s, &distance) in steps.iter().enumerate() {
                    if !visited[r][c][d as usize][s] && distance < min_distance {
                        (min_row, min_col, min_dir, min_steps) = (r, c, d, s);
                        min_distance = distance;
                    }
                }
            }
        }
    }
    (min_row, min_col, min_dir, min_steps)
}
