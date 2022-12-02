use std::fs;

fn main() {
    let input = fs::read_to_string("../input/02.txt").unwrap();
    let mut points1 = 0;
    let mut points2 = 0;
    for line in input.lines() {
        let moves: Vec<_> = line.split(' ').collect();
        let their_move = moves[0];
        let me = moves[1];
        points1 += calculate_my_points(their_move, me);
        points2 += calculate_my_points_p2(their_move, me);
    }
    println!("{}", points1);
    println!("{}", points2);
}

fn calculate_my_points(their_move: &str, my_move: &str) -> u32 {
    let mut points = 0;

    // let byte: u8 = my_move.chars().next().unwrap() as u8;
    // points += byte - b'W';

    if my_move == "X" {
        points += 1;
        if their_move == "A" {
            points += 3;
        } else if their_move == "C" {
            points += 6;
        }
    } else if my_move == "Y" {
        points += 2;
        if their_move == "B" {
            points += 3;
        } else if their_move == "A" {
            points += 6;
        }
    } else {
        points += 3;
        if their_move == "C" {
            points += 3;
        } else if their_move == "B" {
            points += 6;
        }
    }
    points
}

fn calculate_my_points_p2(their_move: &str, outcome: &str) -> u32 {
    let my_move = if their_move == "A" {
        if outcome == "X" {
            "Z"
        } else if outcome == "Y" {
            "X"
        } else {
            "Y"
        }
    } else if their_move == "B" {
        if outcome == "X" {
            "X"
        } else if outcome == "Y" {
            "Y"
        } else {
            "Z"
        }
    } else if outcome == "X" {
        "Y"
    } else if outcome == "Y" {
        "Z"
    } else {
        "X"
    };

    calculate_my_points(their_move, my_move)
}
