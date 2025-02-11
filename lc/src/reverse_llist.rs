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

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut curr = head;
    let mut prev = None;
    while let Some(mut node) = curr {
        curr = node.next;
        node.next = prev;
        prev = Some(node);
    }
    prev
}

type Node = Option<Box<ListNode>>;

pub fn reverse_list_recursive(head: Node) -> Node {
    fn recurse(curr: Node, prev: Node) -> Node {
        match curr {
            Some(mut curr_node) => {
                let next = curr_node.next;
                curr_node.next = prev;
                recurse(next, Some(curr_node))
            }
            None => prev,
        }
    }
    recurse(head, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_two_nodes() {
        let mut node1 = ListNode::new(1);
        node1.next = Some(Box::new(ListNode::new(2)));
        let reversed = reverse_list(Some(Box::new(node1))).unwrap();
        assert_eq!(reversed.val, 2);
        let next_node = reversed.next.unwrap();
        assert_eq!(next_node.val, 1);
        assert!(next_node.next.is_none());
    }

    #[test]
    fn test_reverse_empty() {
        let reversed = reverse_list(None);
        assert!(reversed.is_none());
    }

    #[test]
    fn test_reverse_two_nodes_recursive() {
        let mut node1 = ListNode::new(1);
        node1.next = Some(Box::new(ListNode::new(2)));
        let reversed = reverse_list_recursive(Some(Box::new(node1))).unwrap();
        assert_eq!(reversed.val, 2);
        let next_node = reversed.next.unwrap();
        assert_eq!(next_node.val, 1);
        assert!(next_node.next.is_none());
    }

    #[test]
    fn test_reverse_empty_recursive() {
        let reversed = reverse_list_recursive(None);
        assert!(reversed.is_none());
    }
}
