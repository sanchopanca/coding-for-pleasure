use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("../input/08.txt").unwrap();
    let mut forest = Vec::new();
    for line in input.lines() {
        let row: Vec<i8> = line
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
        forest.push(row);
    }

    let mut visible_trees = HashSet::new();
    for i in 0..forest.len() {
        let mut highest = -1;
        for j in 0..forest[i].len() {
            if forest[i][j] > highest {
                highest = forest[i][j];
                visible_trees.insert((i, j));
            }
        }

        highest = -1;
        for j in (0..forest[i].len()).rev() {
            if forest[i][j] > highest {
                highest = forest[i][j];
                visible_trees.insert((i, j));
            }
        }
    }

    for j in 0..forest[0].len() {
        let mut highest = -1;
        for i in 0..forest.len() {
            if forest[i][j] > highest {
                highest = forest[i][j];
                visible_trees.insert((i, j));
            }
        }

        highest = -1;
        for i in (0..forest.len()).rev() {
            if forest[i][j] > highest {
                highest = forest[i][j];
                visible_trees.insert((i, j));
            }
        }
    }

    println!("{}", visible_trees.len());

    let mut best_score = 0;
    for i in 1..(forest.len() - 1) {
        for j in 1..(forest[i].len() - 1) {
            let tree = forest[i][j];
            let mut a = 0;
            for ii in (0..i).rev() {
                a += 1;
                if forest[ii][j] >= tree {
                    break;
                }
            }

            let mut b = 0;
            for ii in (i + 1)..forest.len() {
                b += 1;
                if forest[ii][j] >= tree {
                    break;
                }
            }

            let mut c = 0;
            for jj in (0..j).rev() {
                c += 1;
                if forest[i][jj] >= tree {
                    break;
                }
            }

            let mut d = 0;
            for jj in (j + 1)..forest[i].len() {
                d += 1;
                if forest[i][jj] >= tree {
                    break;
                }
            }
            if a * b * c * d > best_score {
                best_score = a * b * c * d;
            }
        }
    }
    println!("{}", best_score)
}
