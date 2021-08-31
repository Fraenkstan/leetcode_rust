use std::collections::HashMap;

pub fn is_unique(astr: String) -> bool {
    let mut map = HashMap::new();
    astr.chars().all(|c| {
        if map.contains_key(&c) {
            return false;
        }
        map.insert(c, 0);
        true
    })
}