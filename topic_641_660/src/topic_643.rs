

pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let mut sum: f64 = f64::MIN;
    let _it = nums.windows(k as usize).for_each(|x|
        sum = sum.max(x.iter().sum::<i32>() as f64)
    );
    sum / k as f64
}

pub fn find_max_average_1(nums: Vec<i32>, k: i32) -> f64 {
    let k = k as usize;
    let mut sum = (0..k).map(|i| nums[i]).sum::<i32>();
    let mut answer = sum as f64 / k as f64;
    for i in k..nums.len() {
        sum = sum + nums[i] - nums[i - k];
        answer = answer.max(sum as f64 / k as f64);
    }
    answer
}