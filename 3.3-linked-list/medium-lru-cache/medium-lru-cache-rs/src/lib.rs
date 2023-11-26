use std::{
    borrow::BorrowMut,
    collections::{HashMap},
};

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
    pub prev: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            prev: None,
            val,
        }
    }
}

struct LRUCache {
    map: HashMap<i32, Box<ListNode>>,
    cache: Option<Box<ListNode>>,
    cap: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            map: HashMap::new(),
            cache: Some(Box::new(ListNode::new(0))),
            cap: capacity,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.map.get(&key) {
            Some(v) => {
                return v.val;
            }
            None => -1,
        }
    }

    fn put(&mut self, key: i32, value: i32) {

        // self.map.insert(key, value);
    }
}
