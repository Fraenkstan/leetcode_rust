pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let m = matrix.len();
    let n = matrix[0].len();
    for i in 0..m / 2 {
        for j in 0..n {
            let temp = matrix[i][j];
            matrix[i][j] = matrix[m - i - 1][j];
            matrix[m - i - 1][j] = temp;
        }
    }
    for i in 0..m {
        for j in 0..i {
            let temp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = temp;
        }
    }
}
