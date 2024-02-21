use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let len = nums.len();
    let map = nums.into_iter().collect::<HashSet<_>>();
    len != map.len()
}

fn main() {
    contains_duplicate(vec![1, 2, 3, 1]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        assert!(contains_duplicate(vec![1, 2, 3, 1]));
        assert!(!contains_duplicate(vec![1, 2, 3, 4]));
    }
}
