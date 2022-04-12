use std::cmp::Ordering;

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len();
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return if target == nums[0] { 1 } else { 0 };
    }
    let (mut l, mut r) = (0, n - 1);
    let mut ans = 0;
    while l <= r {
        let mid = l + (r - l) / 2;
        match target.cmp(&nums[mid]) {
            Ordering::Less => {
                if r == mid {
                    break;
                }
                r = mid
            }
            Ordering::Greater => l = mid + 1,
            Ordering::Equal => {
                let mut less = mid;
                let mut greater = mid;
                loop {
                    if nums[less] == target {
                        ans += 1;
                    }
                    if less == 0 {
                        break;
                    }
                    less -= 1;
                }
                while greater < n && nums[greater] == target {
                    ans += 1;
                    greater += 1;
                }
                ans -= 1;
                break;
            }
        }
    }
    ans
}
