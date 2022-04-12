pub fn majority_element(nums: Vec<i32>) -> i32 {
    let (mut count, mut ret, len) = (1, nums[0], nums.len());
    for i in 1..len {
        count = if nums[i] == ret { count + 1 } else { count - 1 };
        if count == 0 {
            ret = nums[i];
            count = 1;
        }
    }
    if nums
        .into_iter()
        .filter(|&x| x == ret)
        .map(|_x| 1)
        .sum::<usize>()
        * 2
        > len
    {
        ret
    } else {
        -1
    }
}
