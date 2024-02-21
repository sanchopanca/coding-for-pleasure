use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    let mut left = HashMap::<char, i32>::new();
    let mut right = HashMap::<char, i32>::new();
    for c in s.chars() {
        *left.entry(c).or_default() += 1;
    }

    for c in t.chars() {
        *right.entry(c).or_default() += 1;
    }

    left == right
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::is_anagram;

    #[test]
    fn test_us_anagram() {
        assert!(is_anagram("anagram".to_string(), "nagaram".to_string()));
        assert!(!is_anagram("rat".to_string(), "car".to_string()));
    }
}
