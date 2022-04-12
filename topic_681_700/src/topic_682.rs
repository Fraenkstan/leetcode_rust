use std::str::FromStr;

pub fn cal_points(ops: Vec<String>) -> i32 {
    let mut scores = vec![];
    for op in ops {
        match op.as_str() {
            "+" => {
                let sum = scores.iter().rev().take(2).sum::<i32>();
                scores.push(sum);
            }
            "D" => {
                let score = *scores.last().unwrap();
                scores.push(score * 2);
            }
            "C" => {
                scores.pop();
            }
            _ => {
                scores.push(i32::from_str(op.as_str()).unwrap());
            }
        }
    }
    scores.iter().sum()
}
