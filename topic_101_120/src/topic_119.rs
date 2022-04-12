pub fn get_row(row_index: i32) -> Vec<i32> {
    if row_index == 0 {
        return vec![1];
    }
    let prev_row = get_row(row_index - 1);
    let mut row = vec![1];
    (1..prev_row.len()).for_each(|i| row.push(prev_row[i - 1] + prev_row[i]));
    row.push(1);
    row
}
