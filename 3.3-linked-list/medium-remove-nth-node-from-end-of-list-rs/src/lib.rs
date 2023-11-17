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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        Self::solution_v1(head, n)
    }

    // runtime: 1ms, memory: 2 MB
    fn solution_v1(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut count = 0;
        let mut head = head;
        let mut cur = head.as_mut();

        // count nodes
        // time complexity: o(n)
        while cur.is_some() {
            cur = cur.unwrap().next.as_mut();
            count += 1;
        }

        if count <= 1 {
            return None;
        }

        if count == n {
            return head.unwrap().next;
        }

        count -= n + 1;
        let mut cur = head.as_mut().unwrap().as_mut();

        // move to the (count - n -1)th node and link it to the second next node or None if next node is None
        // time complexity: o(n)
        loop {
            if count <= 0 {
                cur.next = match &mut cur.next {
                    Some(n) => n.next.take(),
                    None => None,
                };
                break;
            }

            cur = cur.next.as_mut().unwrap();
            count -= 1;
        }

        head
    }
}
