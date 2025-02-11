use std::collections::HashSet;

pub fn lcsubsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    subsets(nums).into_iter().collect()
}

pub fn subsets(nums: Vec<i32>) -> HashSet<Vec<i32>> {
    let mut empty = HashSet::new();
    empty.insert(Vec::new());
    subsets_iter(empty, nums.as_slice())
}

pub fn subsets_iter(subsets: HashSet<Vec<i32>>, nums: &[i32]) -> HashSet<Vec<i32>> {
    if nums.is_empty() {
        return subsets;
    }
    let mut new_subsets = subsets.clone();
    for mut s in subsets {
        s.push(nums[0]);
        new_subsets.insert(s);
    }
    subsets_iter(new_subsets, &nums[1..])
}

fn bt_subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ssets = Vec::new();
    subsets_backtrack(nums.as_slice(), 0, &mut vec![], &mut ssets);
    ssets
}

fn subsets_backtrack(
    nums: &[i32],
    current_idx: usize,
    current_subset: &mut Vec<i32>,
    subsets: &mut Vec<Vec<i32>>,
) {
    if current_idx >= nums.len() {
        subsets.push(current_subset.to_owned());
        return;
    }
    current_subset.push(nums[current_idx]);
    subsets_backtrack(nums, current_idx + 1, current_subset, subsets);
    current_subset.pop();
    subsets_backtrack(nums, current_idx + 1, current_subset, subsets);
}

pub fn rec_subsets(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() == 0 {
        return vec![vec![]];
    }
    let last = nums.pop().unwrap();
    let remain_subsets = rec_subsets(nums);
    let mut res = remain_subsets.clone();
    for mut v in remain_subsets {
        v.push(last);
        res.push(v);
    }
    res
}

pub fn loop_subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut sets = vec![vec![]];
    for num in nums {
        for mut set in sets.clone() {
            set.push(num);
            sets.push(set);
        }
    }
    sets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsets() {
        let nums = vec![1, 2, 3];
        let expected: HashSet<Vec<i32>> = [
            vec![],
            vec![1],
            vec![2],
            vec![3],
            vec![1, 2],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ]
        .iter()
        .cloned()
        .collect();
        assert_eq!(subsets(nums), expected);
    }

    #[test]
    fn test_bt_subsets() {
        let nums = vec![1, 2, 3];
        let mut result = bt_subsets(nums);
        // Sort for consistent comparison
        result.sort();
        let mut expected = vec![
            vec![],
            vec![1],
            vec![2],
            vec![3],
            vec![1, 2],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];
        expected.sort();
        assert_eq!(result, expected);
    }
}
