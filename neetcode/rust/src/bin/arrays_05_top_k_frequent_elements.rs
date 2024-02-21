use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for num in nums {
        *map.entry(num).or_default() += 1;
    }

    let mut reverse_map = HashMap::new();

    for (k, v) in map {
        reverse_map.insert(v, k);
    }

    let mut keys = reverse_map.keys().copied().collect::<Vec<_>>();
    keys.sort_by(|a, b| b.cmp(a));

    let keys = keys.into_iter().take(k as usize).collect::<Vec<_>>();

    keys.into_iter().map(|k| reverse_map[&k]).collect()
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::top_k_frequent;

    #[test]
    fn test_top_k_frequent() {
        let mut result = top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
        result.sort();
        assert_eq!(result, vec![1, 2]);

        assert_eq!(top_k_frequent(vec![1], 1), vec![1]);
    }
}
