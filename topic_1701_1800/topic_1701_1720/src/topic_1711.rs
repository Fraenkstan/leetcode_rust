use std::collections::HashMap;

pub fn count_pairs(deliciousness: Vec<i32>) -> i32 {
    let mut delicious_map: HashMap<i32, usize> = HashMap::new();
    deliciousness.iter().for_each(|i| {
        *delicious_map.entry(*i).or_insert(0) += 1;
    });
    let mut ans = 0;
    let mut delicious_vec: Vec<&i32> = delicious_map.keys().into_iter().collect::<Vec<&i32>>();
    delicious_vec.sort();
    for i in 0..(delicious_vec.len()) {
        for j in if *delicious_map.get(delicious_vec[i]).unwrap() > 1 {
            i..delicious_vec.len()
        } else {
            (i + 1)..delicious_vec.len()
        } {
            if (0..22)
                .into_iter()
                .any(|k| 2_i32.pow(k) == (delicious_vec[i] + delicious_vec[j]) as i32)
            {
                if i == j {
                    let count = delicious_map.get(delicious_vec[i]).unwrap();
                    ans += count * (count - 1) / 2;
                } else {
                    ans += delicious_map.get(delicious_vec[i]).unwrap()
                        * delicious_map.get(delicious_vec[j]).unwrap();
                }
            }
        }
    }
    ans as i32
}

const MOD: i32 = 1000000007;

pub fn count_pairs_1(deliciousness: Vec<i32>) -> i32 {
    let mut m = HashMap::new();
    deliciousness.into_iter().fold(0, |ans, x| {
        (
            (0..22)
                .map(|i| (1 << i) - x)
                .filter_map(|t| m.get(&t))
                .fold(ans, |acc, cnt| (acc + cnt) % MOD),
            *m.entry(x).or_insert(0) += 1,
        )
            .0
    })
}
