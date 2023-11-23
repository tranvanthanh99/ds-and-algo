// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
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

// runtime: 0ms, memory: 2.1 MB
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut l1 = l1;
    let mut l2 = l2;
    let mut out = Box::new(ListNode::new(0));
    let mut tail = &mut out;

    let mut sum = 0;

    // o(n)
    loop {
        if l1.is_none() && l2.is_none() {
            if sum == 1 {
                tail.next = Some(Box::new(ListNode::new(1)));
            }
            break;
        }

        let v1 = match l1.as_ref() {
            Some(n) => n.val,
            None => 0,
        };

        let v2 = match l2.as_ref() {
            Some(n) => n.val,
            None => 0,
        };

        sum += v1 + v2;

        tail.next = Some(Box::new(ListNode::new(sum % 10)));
        tail = tail.next.as_mut().unwrap();

        sum /= 10;

        if l1.is_some() {
            l1 = l1.unwrap().next;
        }
        if l2.is_some() {
            l2 = l2.unwrap().next;
        }
    }

    out.next
}
