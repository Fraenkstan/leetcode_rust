use std::collections::HashSet;

pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
    let n = nums.len();
    if n < 2 {
        return false;
    }
    let mut pre = HashSet::with_capacity(n + 1);
    let mut buf = -1;
    let mut cur = 0;
    let result = nums.iter().any(|num| {
        if pre.contains(&cur) {
            return true;
        }
        pre.insert(buf);
        buf = cur;
        cur = (cur + num) % k;
        false
    });
    if !result {
        return pre.contains(&cur);
    }
    result
}
