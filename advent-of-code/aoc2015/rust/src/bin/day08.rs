use std::fs;

use fancy_regex::Regex;

fn main() {
    let input = fs::read_to_string("../input/08.txt").unwrap();
    let re1 = Regex::new(r#"(\\\\|\\")"#).unwrap();
    let re2 = Regex::new(r"\\x([0-9a-f]){2}").unwrap();
    let mut res1 = 0;
    let mut res2 = 0;
    for line in input.lines() {
        let x1 = re1.find_iter(line).count();
        let x2 = re2.find_iter(line).count();
        res1 += 2 + x1 + x2 * 3;
        res2 += 4 + x1 * 2 + x2;
    }
    println!("{}", res1);
    println!("{}", res2);
}
