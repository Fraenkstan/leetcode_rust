pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len();
    let (mut left, mut right) = (0 as usize, n - 1);
    while left <= right {
        let mid = (right - left) / 2 + left;
        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] < nums[right] {
            if nums[mid] < target && target <= nums[right] {
                left = mid + 1;
            } else {
                if mid == 0 {
                    return -1;
                } else {
                    right = mid - 1
                }
            }
        } else {
            if nums[left] <= target && target < nums[mid] {
                if mid == 0 {
                    return -1;
                }
                right = mid - 1;
            } else {
                left = mid + 1
            }
        }
    }
    -1
}
