pub fn title_to_number(column_title: String) -> i32 {
    column_title
        .chars()
        .into_iter()
        .rev()
        .enumerate()
        .fold(0, |mut acc, (i, c)| {
            acc += (c as i32 - '@' as i32) * 26_i32.pow(i as u32);
            acc
        })
}
