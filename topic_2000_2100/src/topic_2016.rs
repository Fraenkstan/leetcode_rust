pub fn maximum_difference(nums: Vec<i32>) -> i32 {
    nums.iter()
        .fold((i32::MAX, -1), |(mut min, mut ans), &num| {
            if num > min {
                ans = ans.max(num - min);
            }
            min = min.min(num);
            (min, ans)
        })
        .1
}
