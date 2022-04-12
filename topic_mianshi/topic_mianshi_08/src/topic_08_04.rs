pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    let mut ans = vec![];
    backtrace(&mut vec![], &mut nums, 0, &mut ans);
    ans
}

fn backtrace(path: &mut Vec<i32>, nums: &Vec<i32>, i: usize, answer: &mut Vec<Vec<i32>>) {
    answer.push(path.clone());
    for j in i..nums.len() {
        path.push(nums[j]);
        backtrace(path, nums, j + 1, answer);
        path.pop();
    }
}
