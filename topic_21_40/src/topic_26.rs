pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as i32
}

pub fn remove_duplicates_1(nums: &mut Vec<i32>) -> i32 {
    match nums.is_empty() {
        true => 0,
        _ => {
            let mut i = 0;
            for j in 1..nums.len() {
                if nums[i] != nums[j] {
                    nums[i + 1] = nums[j];
                    i += 1;
                }
            }
            (i + 1) as i32
        }
    }
}
