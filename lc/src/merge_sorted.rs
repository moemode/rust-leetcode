use std::{
    mem::{replace, swap},
    ops::Deref,
};

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn merge_two_lists(
    mut left: Option<Box<ListNode>>,
    mut right: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut merged_upto = &mut left;
    let remaining = &mut right;
    while remaining.is_some() {
        if merged_upto.is_none()
            || merged_upto.as_ref().unwrap().val > remaining.as_ref().unwrap().val
        {
            swap(merged_upto, remaining);
        }
        merged_upto = &mut merged_upto.as_mut().unwrap().next;
    }
    left
}

pub fn inelegant_merge_two_lists(
    mut left: Option<Box<ListNode>>,
    mut right: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode::new(0)));
    let mut tail = &mut dummy;

    while let Some(node) = get_next(&mut left, &mut right) {
        tail.as_mut().unwrap().next = Some(node);
        tail = &mut tail.as_mut().unwrap().next;
    }

    dummy.unwrap().next
}

fn get_next(
    left: &mut Option<Box<ListNode>>,
    right: &mut Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (left.take(), right.take()) {
        (Some(mut lnode), Some(mut rnode)) => {
            if lnode.val < rnode.val {
                *left = lnode.next.take();
                *right = Some(rnode);
                Some(lnode)
            } else {
                *right = rnode.next.take();
                *left = Some(lnode);
                Some(rnode)
            }
        }
        (Some(mut lnode), None) => {
            *left = lnode.next.take();
            Some(lnode)
        }
        (None, Some(mut rnode)) => {
            *right = rnode.next.take();
            Some(rnode)
        }
        (None, None) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to convert a slice into a linked list.
    fn to_list(nums: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in nums.iter().rev() {
            let mut node = Box::new(ListNode::new(val));
            node.next = head;
            head = Some(node);
        }
        head
    }

    // Helper to convert a linked list into a vector.
    fn to_vec(mut list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = Vec::new();
        while let Some(node) = list {
            vec.push(node.val);
            list = node.next;
        }
        vec
    }

    #[test]
    fn test_merge_lists_example1() {
        let list1 = to_list(&[1, 2, 4]);
        let list2 = to_list(&[1, 3, 4]);
        let merged = merge_two_lists(list1, list2);
        assert_eq!(to_vec(merged), vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn test_merge_lists_example2() {
        let list1 = to_list(&[]);
        let list2 = to_list(&[]);
        let merged = merge_two_lists(list1, list2);
        assert_eq!(to_vec(merged), Vec::<i32>::new());
    }

    #[test]
    fn test_merge_lists_example3() {
        let list1 = to_list(&[]);
        let list2 = to_list(&[0]);
        let merged = merge_two_lists(list1, list2);
        assert_eq!(to_vec(merged), vec![0]);
    }
}
