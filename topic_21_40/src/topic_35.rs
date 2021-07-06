

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len();
    if n == 0 { return 0 }
    let (mut l, mut r) = (0 as usize, n - 1);
    if target > nums[r] { return n as i32 }
    if target < nums[l] { return 0 }
    while l <= r {
        if l == r {
            return if target == nums[l] { l as i32 }
            else if target < nums[l] { l as i32 }
            else { l as i32 + 1 }
        }
        let mid = (r - l) / 2 + l;
        if nums[mid] == target {
            return mid as i32
        } else if nums[mid] > target {
            r = mid;
        } else { l = mid + 1 }
    }
    -1
}