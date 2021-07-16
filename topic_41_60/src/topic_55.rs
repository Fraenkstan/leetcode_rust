

pub fn can_jump(nums: Vec<i32>) -> bool {
    let (mut max_index, mut index) = (0, 0);
    while index <= max_index && index < nums.len() {
        max_index = (index + nums[index] as usize).max(max_index);
        index += 1;
    }
    max_index >= nums.len() - 1
}