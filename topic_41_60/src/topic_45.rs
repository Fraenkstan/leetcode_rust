pub fn jump(nums: Vec<i32>) -> i32 {
    let (mut max_index, mut index, mut end, mut steps) = (0, 0, 0, 0);
    while index <= max_index && index < nums.len() && end < nums.len() - 1 {
        max_index = max_index.max(index + nums[index] as usize);
        if index == end {
            end = max_index;
            steps += 1;
        }
        index += 1;
    }
    steps
}
