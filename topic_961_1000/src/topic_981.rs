use std::collections::HashMap;

pub struct TimeMap {
    data: HashMap<String, (Vec<i32>, HashMap<i32, String>)>,
}

impl TimeMap {
    pub fn new() -> Self {
        TimeMap {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String, timestamp: i32) {
        let t = self
            .data
            .entry(key)
            .or_insert_with(|| (Vec::new(), HashMap::new()));
        match t.0.binary_search(&timestamp) {
            Ok(i) => t.0[i] = timestamp,
            Err(i) => t.0.insert(i, timestamp),
        }
        t.1.insert(timestamp, value);
    }

    pub fn get(&self, key: String, timestamp: i32) -> String {
        if let Some((v, m)) = self.data.get(&key) {
            match v.binary_search(&timestamp) {
                Ok(_) => m[&timestamp].clone(),
                Err(i) => {
                    if i > 0 {
                        m[&v[i - 1]].clone()
                    } else {
                        String::new()
                    }
                }
            }
        } else {
            String::new()
        }
    }
}
