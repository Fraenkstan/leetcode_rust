use std::collections::HashMap;

pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut map = HashMap::with_capacity(nums.len());
    map.insert(0, 1);
    nums.iter()
        .fold((0, 0), |(mut cur, mut ans), num| {
            cur += num;
            ans += *map.get(&(cur - k)).unwrap_or(&0);
            *map.entry(cur).or_insert(0) += 1;
            (cur, ans)
        })
        .1
}
