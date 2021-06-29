
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    nums.iter().fold((0, std::i32::MIN), |mut acc, &x| {
        acc.0 = core::cmp::max(acc.0 + x, x);
        acc.1 = core::cmp::max(acc.0, acc.1);
        acc
    }).1
}