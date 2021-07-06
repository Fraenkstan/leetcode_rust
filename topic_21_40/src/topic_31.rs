

pub fn next_permutation(nums: &mut Vec<i32>) {
    let n = nums.len();
    for i in (0..n - 1).rev() {
        if nums[i] < nums[i + 1] {
            let mut swap_at = n - 1;
            while nums[i] >= nums[swap_at] { swap_at -= 1; }
            nums.swap(i, swap_at);
            nums[i + 1..].reverse();
            return;
        }
    }
    nums.reverse();
}