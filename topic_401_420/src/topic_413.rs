

pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n < 3 {
        return 0;
    }
    let (mut ans, mut d, mut acc) = (0, nums[0] - nums[1], 0);
    for i in 2..n {
        if nums[i - 1] - nums[i] == d {
            acc += 1;
        } else {
            d = nums[i - 1] - nums[i];
            acc = 0;
        }
        ans += acc;
    }
    ans
}