


pub fn majority_element(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold((0, 0), |(mut acc, mut candidate), num| {
        if acc == 0 { candidate = num }
        if candidate == num { acc += 1 } else { acc -= 1 }
        (acc, candidate)
    }).1
}