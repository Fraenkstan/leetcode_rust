pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ret = Vec::new();
    let (m, n) = (matrix.len(), matrix[0].len());
    for i in 0..m {
        let mut min = matrix[i][0];
        let mut min_idx = 0;
        for j in 1..n {
            if min > matrix[i][j] {
                min = matrix[i][j];
                min_idx = j;
            }
        }
        let mut max = min;
        for j in 0..m {
            max = max.max(matrix[j][min_idx]);
        }
        if min == max {
            ret.push(max);
        }
    }
    ret
}
