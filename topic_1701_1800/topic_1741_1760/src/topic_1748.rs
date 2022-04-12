use std::collections::HashMap;

pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
    let mut cnt = HashMap::new();
    nums.iter().for_each(|x| *cnt.entry(*x).or_insert(0) += 1);
    cnt.iter()
        .filter(|(_, v)| **v == 1)
        .map(|(k, _)| *k)
        .sum::<i32>()
}
