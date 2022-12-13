use std::fs;
use std::cmp;

use itertools::Itertools;

use serde_json::Value;

fn main() {
    let input = fs::read_to_string("../input/13.txt").unwrap().trim().to_owned();
    // let input = fs::read_to_string("13.txt").unwrap().trim().to_owned();
    let mut index = 1;
    let mut res = 0;
    for pair in input.split("\n\n") {
        let (s1, s2) = pair.split('\n').collect_tuple().unwrap();
        let left: Value = serde_json::from_str(s1).unwrap();
        let left = left.as_array().unwrap();
        
        let right: Value = serde_json::from_str(s2).unwrap();
        let right = right.as_array().unwrap();

        if in_right_order(left, right) {
            res += index;
            // println!("{}", index);
        }
        index += 1;
    }
    println!("{}", res);
}

fn in_right_order(left: &Vec<Value>, right: &Vec<Value>) -> bool {
    for i in 0..cmp::min(left.len(), right.len()) {
        let (l, r) = (&left[i], &right[i]);
        if left[i].is_i64() && right[i].is_i64() {
            let (l, r) = (l.as_i64().unwrap(), r.as_i64().unwrap());
            if l < r {
                return true;
            } else if l > r {
                return false;
            } else {
                continue;
            }
        } else if left[i].is_array() && right[i].is_array() {
            return in_right_order(left[i].as_array().unwrap(), right[i].as_array().unwrap());
        } else if left[i].is_i64() {
            return in_right_order(&vec![left[i].clone()], right[i].as_array().unwrap());
        } else {
            return in_right_order(left[i].as_array().unwrap(), &vec![right[i].clone()]);
        }
    }
    return left.len() <= right.len(); // < makes the final result too low, <= makes it to high. <= is the right one for test input
}
