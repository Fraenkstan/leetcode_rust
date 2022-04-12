use std::cmp::Ordering;

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len();
    let (mut left, mut right) = (0, n - 1);
    while left <= right {
        if left == right {
            return if nums[left] == target {
                left as i32
            } else {
                -1
            };
        }
        let mid = left + (right - left) / 2;
        match target.cmp(&nums[mid]) {
            Ordering::Less => right = mid,
            Ordering::Greater => left = mid + 1,
            Ordering::Equal => return mid as i32,
        }
    }
    -1
}
