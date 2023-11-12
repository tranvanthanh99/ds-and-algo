pub struct Solution {}

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
        let mut pre: Option<Box<ListNode>> = None;
        let mut cur = head;

        while let Some(mut node) = cur {
            let next = node.next;
            node.next = pre;
            pre = Some(node);
            cur = next;
        }

        pre
    }
}
