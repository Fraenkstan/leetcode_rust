

pub fn find_magic_index(nums: Vec<i32>) -> i32 {
    nums.into_iter().enumerate().filter(|(i, num)| *i as i32 == *num)
        .next().unwrap_or((0, -1)).1
}