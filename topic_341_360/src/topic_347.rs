use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    nums.into_iter().for_each(|num| {
        *map.entry(num).or_insert(0) += 1;
    });
    let mut keys = map.keys().collect::<Vec<&i32>>();
    keys.sort_by(|&a, &b| map.get(b).cmp(&map.get(a)));
    keys.into_iter().take(k as usize).map(|i| *i).collect()
}
