pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    (0..matrix.first().map_or(0, |r| r.len()))
        .map(|c| matrix.iter().map(|r| r[c]).collect())
        .collect()
}
