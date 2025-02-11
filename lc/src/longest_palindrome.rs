use std::{cmp::max, collections::HashMap};

pub fn longest_palindrome(s: String) -> i32 {
    let mut counts = HashMap::<char, i32>::new();
    for char in s.chars() {
        *counts.entry(char).or_insert(0) += 1;
    }
    let mut nmid = 0;
    let mut palindrome_length = 0;
    for (_, count) in counts {
        let rem = count % 2;
        nmid = max(nmid, rem);
        palindrome_length += count - rem;
    }
    palindrome_length + nmid
}

pub fn longest_palindrome_clever(s: String) -> i32 {
    let mut counts = HashMap::<char, i32>::new();
    let mut odd_occurence_count = 0;
    for ch in s.chars() {
        let count = counts.entry(ch).or_insert(0);
        *count += 1;
        odd_occurence_count += if *count % 2 == 0 { -1 } else { 1 };
    }
    s.len() as i32 - max(0, odd_occurence_count - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiple_chars() {
        assert_eq!(longest_palindrome("abccccdd".to_string()), 7);
    }

    #[test]
    fn test_single_char() {
        assert_eq!(longest_palindrome("a".to_string()), 1);
    }

    #[test]
    fn test_multiple_chars_clever() {
        assert_eq!(longest_palindrome_clever("abccccdd".to_string()), 7);
    }

    #[test]
    fn test_single_char_clever() {
        assert_eq!(longest_palindrome_clever("a".to_string()), 1);
    }
}
