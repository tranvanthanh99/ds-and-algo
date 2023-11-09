use std::collections::HashMap;

struct TimeMap {
    map: HashMap<String, Vec<(i32, String)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */
impl TimeMap {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map.entry(key).or_default().push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        match self.map.get(&key) {
            Some(v_list) => {
                if timestamp < v_list[0].0 {
                    return "".to_string();
                }
                let mut l = 0;
                let mut r = v_list.len() - 1;

                while l < r {
                    let m = (l + r) / 2;

                    if v_list[m + 1].0 <= timestamp {
                        l = m + 1;
                    } else {
                        r = m;
                    }
                }

                return v_list[r].1.clone();
            }
            None => return "".to_string(),
        }
    }
}
