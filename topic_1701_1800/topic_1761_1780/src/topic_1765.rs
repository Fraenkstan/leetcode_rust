use std::collections::VecDeque;

pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (rows, cols) = (is_water.len(), is_water[0].len());
    let mut visited = vec![vec![false; cols]; rows];
    let mut q = VecDeque::new();
    for i in 0..rows {
        for j in 0..cols {
            if is_water[i][j] == 1 {
                visited[i][j] = true;
                q.push_back((i, j));
            }
        }
    }
    let mut ans = vec![vec![0; cols]; rows];
    let mut high = 0;
    while !q.is_empty() {
        high += 1;
        for _ in 0..q.len() {
            let (i, j) = q.pop_front().unwrap();
            for (x, y) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
                if x < rows && y < cols && !visited[x][y] {
                    ans[x][y] = high;
                    visited[x][y] = true;
                    q.push_back((x,y));
                }
            }
        }
    }
    ans
}