

pub fn best_rotation(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut score = (0..n).fold(vec![0; n], |mut score, i| {
        score[(i - nums[i] as usize + 1 + n) % n] -= 1;
        score
    });
    (1..n).fold(0, |max_idx, i| {
        score[i] += score[i - 1] + 1;
        if score[i] > score[max_idx as usize] { i as i32 } else { max_idx }
    })
}