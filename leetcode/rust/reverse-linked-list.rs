// https://leetcode.com/problems/reverse-linked-list/

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

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut val_vec: Vec<i32> = Vec::new();

        let mut current_node = head;

        while let Some(node) = current_node.as_ref() {
            val_vec.push(node.val);
            current_node = node.next.clone();
        }

        if val_vec.is_empty() {
            return None;
        }

        val_vec.reverse();

        let mut last_node = ListNode::new(val_vec[0]);
        let mut current_node = &mut last_node;

        for &item in val_vec.iter().skip(1) {
            let new_node = ListNode::new(item);
            {
                current_node.next = Some(Box::new(new_node));
                current_node = current_node.next.as_mut().unwrap();
            }
        }

        Some(Box::new(last_node))
    }
}
