pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
    let (mut l, mut r) = (0, nums.len() - 1);
    while l < r {
        let m = l + ((r - l) >> 1);
        if m % 2 == 0 && nums[m] == nums[m + 1] || m % 2 == 1 && nums[m - 1] == nums[m] {
            l = m + 1;
        } else {
            r = m;
        }
    }
    nums[r]
}
