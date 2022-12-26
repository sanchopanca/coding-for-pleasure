use std::cmp;
use std::cmp::Ordering;
use std::fs;

use itertools::Itertools;

use serde_json::Value;

fn main() {
    let input = fs::read_to_string("../input/13.txt")
        .unwrap()
        .trim()
        .to_owned();
    let mut index = 1;
    let mut res = 0;
    let mut packets = vec![];
    for pair in input.split("\n\n") {
        let (s1, s2) = pair.split('\n').collect_tuple().unwrap();
        let left: Value = serde_json::from_str(s1).unwrap();
        let left = left.as_array().unwrap();

        let right: Value = serde_json::from_str(s2).unwrap();
        let right = right.as_array().unwrap();

        if compare(left, right) == Ordering::Less {
            res += index;
        }
        index += 1;

        packets.push(left.to_owned());
        packets.push(right.to_owned());
    }
    println!("{}", res);

    packets.push(serde_json::from_str("[[2]]").unwrap());
    packets.push(serde_json::from_str("[[6]]").unwrap());

    packets.sort_by(compare);

    let i1 = packets
        .iter()
        .position(|x| compare(x, &serde_json::from_str("[[2]]").unwrap()) == Ordering::Equal)
        .unwrap()
        + 1;
    let i2 = packets
        .iter()
        .position(|x| compare(x, &serde_json::from_str("[[6]]").unwrap()) == Ordering::Equal)
        .unwrap()
        + 1;

    println!("{}", i1 * i2);
}

fn compare(left: &Vec<Value>, right: &Vec<Value>) -> Ordering {
    for i in 0..cmp::min(left.len(), right.len()) {
        let (l, r) = (&left[i], &right[i]);
        if left[i].is_i64() && right[i].is_i64() {
            let (l, r) = (l.as_i64().unwrap(), r.as_i64().unwrap());
            if l < r {
                return Ordering::Less;
            } else if l > r {
                return Ordering::Greater;
            } else {
                continue;
            }
        } else if left[i].is_array() && right[i].is_array() {
            let res = compare(left[i].as_array().unwrap(), right[i].as_array().unwrap());
            if res == Ordering::Equal {
                continue;
            }
            return res;
        } else if left[i].is_i64() {
            let res = compare(&vec![left[i].clone()], right[i].as_array().unwrap());
            if res == Ordering::Equal {
                continue;
            }
            return res;
        } else {
            let res = compare(left[i].as_array().unwrap(), &vec![right[i].clone()]);
            if res == Ordering::Equal {
                continue;
            }
            return res;
        }
    }
    if left.len() < right.len() {
        Ordering::Less
    } else if left.len() > right.len() {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}
