pub fn count_bits(num: i32) -> Vec<i32> {
    (1..=num as usize).fold(vec![0], |mut v, i| {
        v.push(&v[i & (i - 1)] + 1);
        v
    })
}
