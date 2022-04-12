pub fn search_matrix(matrix: &Vec<Vec<i32>>, target: i32) -> bool {
    let (mut x, mut y) = (matrix.len() - 1, 0);
    let len = matrix[0].len();
    while x > 0 && y < len {
        if matrix[x][y] == target {
            return true;
        } else if matrix[x][y] > target {
            x -= 1;
        } else {
            y += 1
        }
    }
    false
}
