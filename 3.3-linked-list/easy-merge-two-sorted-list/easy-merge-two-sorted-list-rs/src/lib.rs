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

pub struct Solution {}

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut list1 = list1;
        let mut list2 = list2;

        let mut pre_head = ListNode::new(0);
        let mut tail = &mut pre_head;

        while list1.is_some() && list2.is_some() {
            if list1.as_ref().unwrap().val < list2.as_ref().unwrap().val {
                tail.next = list1.take();
                tail = tail.next.as_mut().unwrap();
                list1 = tail.next.take();
            } else {
                tail.next = list2.take();
                tail = tail.next.as_mut().unwrap();
                list2 = tail.next.take();
            }
        }

        if list1.is_some() {
            tail.next = list1.take();
        }

        if list2.is_some() {
            tail.next = list2.take();
        }

        pre_head.next
    }
}
