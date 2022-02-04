
pub fn number_of_weak_characters(mut properties: Vec<Vec<i32>>) -> i32 {
    properties.sort_unstable_by(|a, b| b[0].cmp(&a[0]).then(a[1].cmp(&b[1])));
    let mut max_def = properties[0][1];
    let mut ans = 0;
    for x in properties.iter() {
        if x[1] > max_def {
            max_def = x[1];
        } else if x[1] < max_def {
            ans += 1;
        }
    }
    ans
}