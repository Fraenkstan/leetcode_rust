pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    if nums.len() < 3 {
        return false;
    }
    let mut first = nums[0];
    let mut second = i32::MAX;
    nums.iter().any(|&num| {
        if num > second {
            return true;
        } else if num > first {
            second = num;
        } else {
            first = num
        }
        false
    })
}
