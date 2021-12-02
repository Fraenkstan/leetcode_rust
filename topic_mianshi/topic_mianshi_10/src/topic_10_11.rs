
pub fn wiggle_sort(nums: &mut Vec<i32>) {
    if nums.len() < 3 {
        return;
    }
    for i in 1..nums.len() {
        if i % 2 == 0 {
            if nums[i] < nums[i - 1] {
                nums.swap(i - 1, i);
            }
        } else if nums[i] > nums[i - 1] {
            nums.swap(i - 1, i);
        }
    }
}