pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let len = nums.len();
    let mut result: Vec<String> = Vec::new();
    if len == 0 {
        return result;
    }
    let mut sub_vecs: Vec<Vec<i32>> = Vec::new();
    let mut sub_vec: Vec<i32> = Vec::new();
    sub_vec.push(nums[0]);
    for i in 1..len {
        if *nums.get(i).unwrap() == *nums.get(i - 1).unwrap() + 1 {
        } else {
            sub_vecs.push(sub_vec);
            sub_vec = Vec::new();
        }
        sub_vec.push(nums[i]);
    }
    sub_vecs.push(sub_vec);
    for sub in sub_vecs {
        if sub.len() == 1 {
            result.push(format!("{}", sub[0]));
        } else {
            result.push(format!("{}->{}", sub[0], sub[sub.len() - 1]));
        }
    }
    result
}
