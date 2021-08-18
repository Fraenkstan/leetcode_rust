
pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
    let mut ans = 0;
    let n = candy_type.len() as i32;
    let mut visited = vec![];
    candy_type.iter().fold(0, |mut acc, x| {
        if acc < n / 2 {
            if visited.contains(x) {
                acc += 1;
            } else {
                ans += 1;
                visited.push(*x);
            }
        } else {
            if !visited.contains(x) {
                ans += 1;
                visited.push(*x);
            }
        }
        acc
    });
    ans.min(n / 2)
}