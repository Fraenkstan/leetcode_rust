

pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    for i in 0..nums.len() - 1 {
        if i == 0 {
            if nums[0] > nums[1] { return 0 }
        }
        else if nums[i - 1] < nums[i] && nums[i] > nums[i + 1] { return i as i32; }
    }
    nums.len() as i32 - 1
}