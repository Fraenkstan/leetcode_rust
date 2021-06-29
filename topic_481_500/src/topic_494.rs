use std::collections::HashMap;

pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    let n = nums.len();
    let mut per_hm = HashMap::new();
    *per_hm.entry(nums[0]).or_insert(0) += 1;
    *per_hm.entry(-nums[0]).or_insert(0) += 1;
    let mut now_hm = HashMap::new();
    for i in 1..n {
        for (k, v) in &per_hm {
            *now_hm.entry(*k + nums[i]).or_insert(0) += *v;
            *now_hm.entry(*k - nums[i]).or_insert(0) += *v;
        }
        per_hm = now_hm.clone();
        now_hm.clear();
    }
    *per_hm.get(&target).unwrap_or(&0)
}

pub fn solution1(nums: Vec<i32>, target: i32) -> i32 {
    let sum = nums.iter().sum::<i32>();
    if (sum - target) % 2 != 0 {
        return 0;
    }
    let neg = (sum - target) / 2;

    target
}
