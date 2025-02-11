use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    i32::try_from(longest_substring_no_repeat(s).len()).unwrap()
}

pub fn longest_substring_no_repeat(s: String) -> String {
    let mut longest_length: usize = 0;
    let mut longest_start: usize = 0;
    let mut longest_suffix_start: usize = 0;
    let mut longest_suffix_chars = HashSet::new();
    let chars = s.chars().collect::<Vec<_>>();
    for (idx, c) in chars.iter().enumerate() {
        if longest_suffix_chars.insert(*c) {
            let suffix_len = (idx - longest_suffix_start) + 1;
            if suffix_len > longest_length {
                longest_length = suffix_len;
                longest_start = longest_suffix_start;
            }
        } else {
            for j in longest_suffix_start..idx {
                longest_suffix_start = j + 1;
                if chars[j] == *c {
                    break;
                }
                longest_suffix_chars.remove(&chars[j]);
            }
        }
    }
    chars[longest_start..longest_start + longest_length]
        .iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abcabcbb() {
        let input = "abcabcbb".to_string();
        let result = longest_substring_no_repeat(input);
        assert_eq!(result.len(), 3);
        assert!(["abc", "bca", "cab"].contains(&result.as_str()));
    }

    #[test]
    fn test_bbbbb() {
        let input = "bbbbb".to_string();
        let result = longest_substring_no_repeat(input);
        assert_eq!(result.len(), 1);
        assert_eq!(result, "b");
    }

    #[test]
    fn test_pwwkew() {
        let input = "pwwkew".to_string();
        let result = longest_substring_no_repeat(input);
        assert_eq!(result.len(), 3);
        assert!(["wke", "kew"].contains(&result.as_str()));
    }

    #[test]
    fn test_empty() {
        let input = "".to_string();
        let result = longest_substring_no_repeat(input);
        assert_eq!(result.len(), 0);
        assert_eq!(result, "");
    }
}
