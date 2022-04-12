pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
    let sum = card_points.iter().sum::<i32>();
    let window_size = card_points.len() - k as usize;
    let mut window_sum = 0;
    if window_size > 0 {
        window_sum = i32::max_value();
        card_points.windows(window_size).for_each(|x| {
            let tmp = x.iter().sum::<i32>();
            if tmp < window_sum {
                window_sum = tmp;
            }
        });
    }
    sum - window_sum
}

#[allow(dead_code)]
pub fn max_score_1(card_points: Vec<i32>, k: i32) -> i32 {
    let windows: usize = card_points.len() - k as usize;
    let mut sum: i32 = card_points[0..windows].iter().sum();
    let mut csum = sum;
    let mut min_sum: i32 = sum;
    for i in windows..card_points.len() {
        sum += card_points[i] - card_points[i - windows];
        csum += card_points[i];
        min_sum = min_sum.min(sum);
    }
    csum - min_sum
}
