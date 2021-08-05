

pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let (mut l, mut res, mut cost) = (0, 1, 0);
    let n = nums.len();
    for r in 1 .. n {
        cost += (nums[r] - nums[r - 1]) * ((r - l) as i32);
        while cost > k {
            cost -= nums[r] - nums[l];
            l += 1;
        }
        res = res.max(r - l + 1);
    }
    res as i32
}