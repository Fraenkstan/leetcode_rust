
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold((0, i32::MIN), |(mut s, ans), i| {
        s = i.max(s + i);
        (s, ans.max(s))
    }).1
}