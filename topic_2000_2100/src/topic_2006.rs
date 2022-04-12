use std::collections::HashMap;

pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
    let mut map = HashMap::<i32, i32>::new();
    let mut ans = 0;
    nums.iter().for_each(|&num| {
        ans += map.get(&(num - k)).unwrap_or(&0) + map.get(&(num + k)).unwrap_or(&0);
        *map.entry(num).or_insert(0) += 1;
    });
    ans
}
