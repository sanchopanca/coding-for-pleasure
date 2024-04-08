pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = Vec::new();
    for token in tokens {
        match token.as_str() {
            "+" | "-" | "*" | "/" => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();

                stack.push(match token.as_str() {
                    "+" => left + right,
                    "-" => left - right,
                    "*" => left * right,
                    "/" => left / right,
                    _ => unreachable!(),
                })
            }
            _ => stack.push(token.parse::<i32>().unwrap()),
        }
    }
    stack.pop().unwrap()
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::eval_rpn;

    #[test]
    fn test_eval_rpn() {
        assert_eq!(
            eval_rpn(vec![
                "2".to_string(),
                "1".to_string(),
                "+".to_string(),
                "3".to_string(),
                "*".to_string()
            ]),
            9
        );
        assert_eq!(
            eval_rpn(vec![
                "4".to_string(),
                "13".to_string(),
                "5".to_string(),
                "/".to_string(),
                "+".to_string()
            ]),
            6
        );
        assert_eq!(
            eval_rpn(vec![
                "10".to_string(),
                "6".to_string(),
                "9".to_string(),
                "3".to_string(),
                "+".to_string(),
                "-11".to_string(),
                "*".to_string(),
                "/".to_string(),
                "*".to_string(),
                "17".to_string(),
                "+".to_string(),
                "5".to_string(),
                "+".to_string()
            ]),
            22
        );
    }
}
