use std::collections::HashSet;

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    if n == 1 {
        return vec!["()".to_string()];
    }

    let mut result = HashSet::new();
    for seq in generate_parenthesis(n - 1) {
        result.insert(format!("(){seq}"));
        result.insert(format!("({seq})"));
        result.insert(format!("{seq}()"));
    }
    let mut result = Vec::from_iter(result);
    result.sort();
    result
}

fn main() {
    println!("{:?}", generate_parenthesis(8));
}

#[cfg(test)]
mod test {
    use crate::generate_parenthesis;

    #[test]
    fn test_generate_parens() {
        assert_eq!(
            generate_parenthesis(3),
            vec![
                "((()))".to_string(),
                "(()())".to_string(),
                "(())()".to_string(),
                "()(())".to_string(),
                "()()()".to_string()
            ]
        );
        assert_eq!(generate_parenthesis(1), vec!["()".to_string()]);
    }
}
