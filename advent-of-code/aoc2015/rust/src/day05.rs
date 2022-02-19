use std::fs;

use fancy_regex::Regex;
use itertools::Itertools;

fn is_nice(s: &str) -> bool {
    let mut vowels = 0;
    vowels += s.matches('a').count();
    vowels += s.matches('e').count();
    vowels += s.matches('i').count();
    vowels += s.matches('o').count();
    vowels += s.matches('u').count();

    let mut has_double = false;
    for (a, b) in s.chars().tuple_windows() {
        if a == b {
            has_double = true;
            break;
        }
    }

    let has_bad_substring = s.contains("ab")
        || s.contains("cd")
        || s.contains("pq")
        || s.contains("xy");

    vowels >= 3 && has_double && !has_bad_substring
    
}

fn is_nice_but_better(s: &str) -> bool {
    let two_pairs_re = Regex::new(r"(..).*(\1)").unwrap();
    let has_two_pairs = two_pairs_re.is_match(s).unwrap();

    let two_letters_re = Regex::new(r"(.).(\1)").unwrap();
    let has_two_letters = two_letters_re.is_match(s).unwrap();

    has_two_pairs && has_two_letters
}


pub fn day05() {
    let strings = fs::read_to_string("../input/05.txt").unwrap();
    let mut nice = 0;
    let mut nice_but_better = 0;
    for s in strings.lines() {
        if is_nice(s) {
            nice += 1;
        }
        if is_nice_but_better(s) {
            nice_but_better += 1;
        }
    }
    println!("{}", nice);
    println!("{}", nice_but_better);
}
