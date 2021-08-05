use std::collections::HashMap;

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    let mut ans = vec![-1, -1];
    numbers.into_iter().enumerate().for_each(|(i, num)| {
        let key = target - num;
        if map.contains_key(&key) {
            ans[0] = map.get(&key).unwrap() + 1;
            ans[1] = i as i32 + 1;
        }
        map.insert(num, i as i32);
    });
    ans
}