

pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let row = matrix.len();
    let col = matrix[0].len();
    let mut row_vec = vec![];
    let mut col_vec = vec![];
    for i in 0..row {
        for j in 0..col {
            if matrix[i][j] == 0 {
                row_vec.push(i);
                col_vec.push(j);
            }
        }
    }
    row_vec.iter().for_each(|&row| {
        for j in 0..col {
            matrix[row][j] = 0
        }
    });
    col_vec.iter().for_each(|&col| {
        for i in 0..row {
            matrix[i][col] = 0
        }
    });
}