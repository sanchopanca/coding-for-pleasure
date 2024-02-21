pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut p = 1;
    for &num in &nums {
        p *= num;
        result.push(p);
    }

    p = 1;
    for i in (0..nums.len()).rev() {
        if i > 0 {
            result[i] = p * result[i - 1];
        } else {
            result[i] = p;
        }
        p *= nums[i];
    }
    result
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::product_except_self;

    #[test]
    fn test_product_except_self() {
        assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
        assert_eq!(
            product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }
}
