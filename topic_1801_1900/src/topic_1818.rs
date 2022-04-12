use std::cmp::max;

pub fn min_absolute_sum_diff(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let len = nums1.len();
    let (mut res, mut max_diff) = (0, 0);
    let mut ss = nums1.clone();
    ss.sort();
    for i in 0..len {
        let cur = (nums1[i] - nums2[i]).abs();
        res += cur as i64;
        let sidx = ss.binary_search(&nums2[i]).unwrap_or_else(|x| x);
        if sidx != 0 {
            max_diff = max(max_diff, cur - (nums2[i] - ss[sidx - 1]));
        }
        if sidx != len {
            max_diff = max(max_diff, cur - (ss[sidx] - nums2[i]));
        }
    }
    res -= max_diff as i64;
    (res % (1e9 as i64 + 7)) as i32
}
