

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let default = vec![-1, -1];
    let n = nums.len();
    if n == 1 {
        return if nums[0] == target { vec![0, 0] } else { default }
    } else if n == 2 {
        return if nums[0] == target && nums[1] == target { vec![0, 1] }
        else if nums[0] == target { vec![0, 0] }
        else if nums[1] == target { vec![1, 1] }
        else { default }
    }
    else if n > 0 {
        let (mut l, mut r) = (0 as usize, n - 1);
        while l <= r {
            let mid = (r - l) / 2 + l;
            if nums[mid] == target {
                l = mid;
                r = mid;
                while r < n - 1 && nums[r + 1] == target {
                    r += 1;
                }
                while l > 0 && nums[l - 1] == target {
                    l -= 1;
                }
                return vec![l as i32, r as i32];
            } else if mid == 0 {
                return default;
            } else if nums[mid] < target {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
    }
    default
}