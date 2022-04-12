pub fn max_number_of_balloons(text: String) -> i32 {
    let mut cache = vec![0; 15];
    text.chars().for_each(|ch| {
        if ['a', 'b', 'l', 'n', 'o'].contains(&ch) {
            cache[ch as usize - 'a' as usize] += 1;
        }
    });
    cache[0]
        .min(cache[1])
        .min(cache[14] / 2)
        .min(cache[13])
        .min(cache[11] / 2)
}
