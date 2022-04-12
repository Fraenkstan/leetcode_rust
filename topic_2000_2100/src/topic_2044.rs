pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
    let sum = nums.iter().fold(0, |acc, &n| acc | n);
    fn dfs(nums: &Vec<i32>, idx: usize, cur: i32, sum: i32) -> i32 {
        if idx == nums.len() - 1 {
            return if cur | nums[idx] == sum { 1 } else { 0 };
        }
        dfs(nums, idx + 1, cur, sum)
            + if cur | nums[idx] == sum {
                2i32.pow((nums.len() - idx) as u32 - 1)
            } else {
                dfs(nums, idx + 1, cur | nums[idx], sum)
            }
    }
    dfs(&nums, 0, 0, sum)
}
