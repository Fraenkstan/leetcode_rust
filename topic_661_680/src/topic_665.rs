pub fn check_possibility(nums: Vec<i32>) -> bool {
    let mut count = 0;
    for i in 0..nums.len() - 1 {
        if nums[i] > nums[i + 1] {
            // 有异常情况
            if count > 0 {
                return false; // 多次异常已经不可能了
            }
            if i > 0 && i < nums.len() - 2 {
                if nums[i - 1] > nums[i + 1] && nums[i] > nums[i + 2] {
                    return false;
                }
            }
            count += 1;
        }
    }
    true
}
