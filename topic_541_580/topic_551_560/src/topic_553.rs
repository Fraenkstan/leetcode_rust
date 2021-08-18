

pub fn optimal_division(nums: Vec<i32>) -> String {
    let n = nums.len();
    let mut ans = nums.iter().enumerate().map(|(i, &num)| {
        match i {
            0 => num.to_string(),
            1 => {
                let mut str = "/".to_string();
                if n > 2 {
                    str.push('(');
                }
                str.push_str(num.to_string().as_str());
                str
            },
            _ => {
                let mut str = "/".to_string();
                str.push_str(num.to_string().as_str());
                str
            }
        }
    }).collect::<String>();
    if n > 2 {
        ans.push(')');
    }
    ans
}