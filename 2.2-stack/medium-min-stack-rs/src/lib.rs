struct MinStack {
    stack: Vec<i32>,
    min: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

// runtime: 6 ms, memory: 6 MB
impl MinStack {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            min: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);

        match self.min.last() {
            Some(last) => {
                if val < *last {
                    self.min.push(val);
                } else {
                    self.min.push(*last);
                }
            }
            None => self.min.push(val),
        }
    }

    fn pop(&mut self) {
        self.min.pop();
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min.last().unwrap()
    }
}
