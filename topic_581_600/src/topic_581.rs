pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let ans = nums.iter().enumerate().fold(
        (i32::max_value(), i32::min_value(), -1, -1),
        |(mut min, mut max, mut left, mut right), (i, &num)| {
            if max > num {
                right = i as i32;
            } else {
                max = num
            }
            if min < nums[n - i - 1] {
                left = (n - i) as i32 - 1;
            } else {
                min = nums[n - i - 1]
            }
            (min, max, left, right)
        },
    );
    if ans.3 == -1 {
        return 0;
    } else {
        ans.3 - ans.2 + 1
    }
}
