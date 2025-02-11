pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    search_option(nums, target).unwrap_or(-1)
}

pub fn search_option(nums: Vec<i32>, target: i32) -> Option<i32> {
    if !is_rotated(&nums) {
        return bin_search(&nums, target).map(|x| x as i32);
    }
    let mid_value = nums[nums.len() / 2];
    let decrease_point = if mid_value > nums[nums.len() - 1] {
        find_decrease_right(&nums)
    } else {
        find_decrease_left(&nums)
    };
    if let Some(decrease_point) = decrease_point {
        if target <= nums[nums.len() - 1] {
            return bin_search(&nums[decrease_point + 1..], target)
                .map(|x| (x + decrease_point + 1) as i32);
        } else {
            return bin_search(&nums[..decrease_point + 1], target).map(|x| x as i32);
        }
    }
    None
}

fn bin_search<A: AsRef<[i32]>>(nums: A, target: i32) -> Option<usize> {
    let nums = nums.as_ref();
    let mut l = 0;
    let mut r = nums.len();
    while l < r {
        let mid = (l + r) / 2;
        match nums[mid].cmp(&target) {
            std::cmp::Ordering::Less => l = mid + 1,
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Greater => r = mid,
        }
    }
    None
}

fn is_rotated<A: AsRef<[i32]>>(nums: A) -> bool {
    let nums = nums.as_ref();
    nums.is_empty() || nums[0] > nums[nums.len() - 1]
}

fn find_decrease_right<A: AsRef<[i32]>>(nums: A) -> Option<usize> {
    let nums = nums.as_ref();
    if nums.is_empty() {
        return None;
    }
    let mut l = (nums.len() - 1) / 2;
    let mut r = nums.len();
    let mut highest_seen = nums[l];
    while l + 1 < r {
        let mid = (l + r) / 2;
        match nums[mid].cmp(&highest_seen) {
            std::cmp::Ordering::Less => {
                r = mid;
            }
            std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                highest_seen = nums[mid];
                l = mid;
            }
        }
    }
    if r >= nums.len() || nums[l] <= nums[r] {
        return None;
    }
    Some(l)
}

fn find_decrease_left<A: AsRef<[i32]>>(nums: A) -> Option<usize> {
    let nums = nums.as_ref();
    if nums.is_empty() {
        return None;
    }
    let mut l = 0;
    let mut r = nums.len() / 2;
    let mut lowest_seen = nums[r];
    while l + 1 < r {
        let mid = (l + r) / 2;
        match nums[mid].cmp(&lowest_seen) {
            std::cmp::Ordering::Less => {
                lowest_seen = nums[mid];
                r = mid;
            }
            std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                l = mid;
            }
        }
    }
    if r >= nums.len() || nums[l] <= nums[r] {
        return None;
    }
    Some(l)
}

#[cfg(test)]
mod tests {

    use crate::search_rotated::find_decrease_right;

    #[test]
    fn test_inc_dec() {
        let input = [3, 5, 6, 7, 0, 2];
        assert_eq!(find_decrease_right(input), Some(3));
    }

    #[test]
    fn test_empty() {
        let input = [];
        assert_eq!(find_decrease_right(input), None);
    }

    #[test]
    fn test_one() {
        let input = [1];
        assert_eq!(find_decrease_right(input), None);
    }

    #[test]
    fn test_two() {
        let input = [2, 1];
        assert_eq!(find_decrease_right(input), Some(0));
    }

    #[test]
    fn test_bin_search_one() {
        let input = [1];
        assert_eq!(super::bin_search(&input, 1), Some(0));
    }

    #[test]
    fn test_find_decrease_left() {
        let input = [6, 7, 0, 1, 2, 4, 5];
        assert_eq!(super::find_decrease_left(input), Some(1));
    }

    #[test]
    fn test_find_decrease_left_empty() {
        let input: [i32; 0] = [];
        assert_eq!(super::find_decrease_left(input), None);
    }

    #[test]
    fn test_find_decrease_left_one() {
        let input = [1];
        assert_eq!(super::find_decrease_left(input), None);
    }

    #[test]
    fn test_find_decrease_left_two() {
        let input = [2, 1];
        assert_eq!(super::find_decrease_left(input), Some(0));
    }

    #[test]
    fn test_find_decrease_left_not_rotated() {
        let input = [1, 2, 3, 4, 5];
        assert_eq!(super::find_decrease_left(input), None);
    }

    #[test]
    fn test_search_option_rotated() {
        let input = vec![4, 5, 6, 7, 0, 1, 2];
        assert_eq!(super::search_option(input.clone(), 0), Some(4));
        assert_eq!(super::search_option(input.clone(), 3), None);
        assert_eq!(super::search_option(input.clone(), 5), Some(1));
    }

    #[test]
    fn test_search_option_small_rotated() {
        let input = vec![3, 1];
        assert_eq!(super::search_option(input.clone(), 3), Some(0));
        assert_eq!(super::search_option(input.clone(), 1), Some(1));
        assert_eq!(super::search_option(input.clone(), 0), None);
    }
}
