

pub fn pile_box(mut box_: Vec<Vec<i32>>) -> i32 {
    let n = box_.len();
    box_.sort_by(|a, b| {
        return if a[0] == b[0] {
            if a[1] == b[1] {
                b[2].cmp(&a[2])
            } else {
                b[1].cmp(&a[1])
            }
        } else {
            a[0].cmp(&b[0])
        }
    });
    println!("{:?}", box_);
    let mut dp = vec![0; n + 1];
    dp[0] = box_[0][2];
    let mut ans = dp[0];
    for i in 1..n {
        let mut tmp = 0;
        for j in 0..i {
            if box_[i][1] > box_[j][1] && box_[i][2] > box_[j][2] {
                tmp = tmp.max(dp[j]);
            }
        }
        dp[i] = tmp + box_[i][2];
        ans = ans.max(dp[i]);
    }
    ans
}

