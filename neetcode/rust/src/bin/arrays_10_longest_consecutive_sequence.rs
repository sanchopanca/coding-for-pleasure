use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let set = nums.into_iter().collect::<HashSet<_>>();

    let mut max = 1;
    for &number in &set {
        if !set.contains(&(number - 1)) {
            let mut i = 1;
            while set.contains(&(number + i)) {
                i += 1
            }
            max = max.max(i);
        }
    }

    max
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::longest_consecutive;

    #[test]
    fn test_longest_consecutive() {
        assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
        assert_eq!(longest_consecutive(vec![]), 0);
    }
}
