use std::collections::HashMap;

pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();
    nums1.into_iter().for_each(|num| {
        map.entry(num).or_insert((true, false));
    });
    nums2.into_iter().for_each(|num| {
        (*map.entry(num).or_insert((false, true))).1 = true;
    });
    map.into_iter().filter(|(_, (nums1, nums2))| *nums1 && *nums2)
        .map(|(k, _)| k).collect()
}