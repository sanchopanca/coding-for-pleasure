pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '{' | '[' => stack.push(c),
            _ => {
                if let Some(previous) = stack.pop() {
                    match c {
                        ')' => {
                            if previous != '(' {
                                return false;
                            }
                        }
                        ']' => {
                            if previous != '[' {
                                return false;
                            }
                        }
                        '}' => {
                            if previous != '{' {
                                return false;
                            }
                        }
                        _ => unreachable!(),
                    }
                } else {
                    return false;
                }
            }
        }
    }

    stack.is_empty()
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::is_valid;

    #[test]
    fn test_is_valid() {
        assert!(is_valid("()".to_string()));
        assert!(is_valid("()[]{}".to_string()));
        assert!(!is_valid("(]".to_string()));
        assert!(is_valid("(())".to_string()));
        assert!(is_valid("[{}]()".to_string()));
        assert!(!is_valid("((".to_string()));
        assert!(!is_valid("))".to_string()));
    }
}
