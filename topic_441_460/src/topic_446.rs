use std::collections::HashMap;

pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let n = nums.len();
    let mut dp = vec![HashMap::<i64, i32>::new(); n];
    for i in 0..n {
        for j in 0..i {
            let div = nums[i] as i64 - nums[j] as i64;
            let cnt = *dp[j].entry(div).or_insert(0);
            ans += cnt;
            *dp[i].entry(div).or_insert(0) += cnt + 1;
        }
    }
    ans
}