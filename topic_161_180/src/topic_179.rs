use std::cmp::Ordering;

pub fn largest_number(nums: Vec<i32>) -> String {
    let mut nums = nums;
    nums.sort_by(|a, b| format!("{}{}", b, a).cmp(&format!("{}{}", a, b)));
    if nums[0] == 0 {
        return 0.to_string();
    }
    nums.into_iter().map(|num| num.to_string()).collect::<String>()
}