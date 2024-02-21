use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut index: HashMap<i32, Vec<usize>> = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        index.entry(num).or_default().push(i);
    }

    for (i, &num) in nums.iter().enumerate() {
        let rest = target - num;
        if let Some(indexes) = index.get(&rest) {
            if let Some(other_index) = indexes.iter().find(|&&idx| idx != i) {
                return vec![i as i32, *other_index as i32];
            }
        }
    }

    unreachable!()
}

fn main() {}

#[cfg(test)]
mod test {
    use super::two_sum;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
