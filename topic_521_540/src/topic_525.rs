use std::collections::HashMap;

pub fn find_max_length(nums: Vec<i32>) -> i32 {
    let mut cur = 0;
    let mut map = HashMap::with_capacity(nums.len());
    map.insert(0, 0);
    let mut ans = 0;
    for (idx, num) in nums.into_iter().enumerate() {
        match num {
            1 => {
                cur += 1;
            }
            _ => {
                cur -= 1;
            }
        }
        if !map.contains_key(&cur) {
            map.insert(cur, idx + 1);
        } else {
            ans = ans.max(idx - map[&cur] + 1);
        }
    }
    ans as i32
}
