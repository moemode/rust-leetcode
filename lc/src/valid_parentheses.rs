//Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' => stack.push(')'),
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            ')' | '}' | ']' => {
                if Some(c) != stack.pop() {
                    return false;
                }
            }
            _ => {} // ignore other characters
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_pair() {
        assert!(is_valid("()".to_string()));
    }

    #[test]
    fn test_multiple_pairs() {
        assert!(is_valid("()[]{}".to_string()));
    }

    #[test]
    fn test_mismatched() {
        assert!(!is_valid("(]".to_string()));
    }

    #[test]
    fn test_nested() {
        assert!(is_valid("([])".to_string()));
    }
}
