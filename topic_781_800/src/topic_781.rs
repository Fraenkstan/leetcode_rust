use std::collections::HashMap;

pub fn num_rabbits(answers: Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    answers.into_iter().for_each(|i| {
        map.entry(i).or_insert(vec![]).push(i);
    });
    map.into_iter().fold(0, |mut ans, (k, v)| {
        let len = v.len();
        if len as i32 <= k + 1 {
            ans += k + 1;
        } else {
            if len as i32 % (k + 1) == 0 {
                ans += (k + 1) * (len as i32 / (k + 1));
            } else { ans += (k + 1) * (len as i32 / (k + 1) + 1) }
        }
        ans
    })
}