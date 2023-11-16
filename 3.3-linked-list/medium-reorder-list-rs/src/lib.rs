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
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        // Self::solution_v1(head);
        Self::solution_v2(head);
    }

    // runtime: 7ms, memory: 5.3 MB
    fn solution_v1(head: &mut Option<Box<ListNode>>) {
        let mut head2 = split_list(head);
        head2 = reverse_list(head2);
        merge_list(head, head2);
    }

    // runtime: 2ms, memory: 2.5 MB
    fn solution_v2(head: &mut Option<Box<ListNode>>) {
        let mut vdq = std::collections::VecDeque::new();
        let mut cur = head.as_mut().unwrap().next.take();

        // add all node to VecDeque - o(n)
        while let Some(mut node) = cur {
            cur = node.next.take();
            vdq.push_back(node);
        }

        let mut back = false;
        let mut cur = head.as_mut().unwrap();

        // pop node from first and last elements in vdq - o(n)
        while !vdq.is_empty() {
            cur.next = match back {
                true => vdq.pop_front(),
                false => vdq.pop_back(),
            };

            cur = cur.next.as_mut().unwrap();
            back = !back;
        }
    }
}

pub fn split_list(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut fast = &head.clone();
    let mut slow = head;

    while let Some(f) = fast {
        fast = &f.next;
        if let Some(f1) = fast {
            fast = &f1.next;
            slow = match slow {
                None => {
                    return None;
                }
                Some(s) => &mut s.next,
            }
        }
    }

    match slow {
        None => {
            return None;
        }
        Some(s) => {
            let head2 = s.next.take();
            s.next = None;
            head2
        }
    }
}

pub fn merge_list(head1: &mut Option<Box<ListNode>>, head2: Option<Box<ListNode>>) {
    let mut h1 = head1;
    let mut h2 = head2;

    while h1.is_some() && h2.is_some() {
        let h1next = h1.as_mut().unwrap().next.take();
        let h2next = h2.as_mut().unwrap().next.take();
        h1.as_mut().unwrap().next = h2;
        h1.as_mut().unwrap().next.as_mut().unwrap().next = h1next;
        h1 = &mut (h1.as_mut().unwrap().next.as_mut().unwrap().next);
        h2 = h2next;
    }
    if h2.is_some() {
        h1 = &mut (h2);
    }
}

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
