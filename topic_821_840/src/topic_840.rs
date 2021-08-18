use std::collections::HashSet;

pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
    let l = grid[0].len();
    let v = grid.len();
    if l < 3 || v < 3 { return 0 }
    let mut ans = 0;
    for i in 0..=v - 3 {
        for j in 0..=l - 3 {
            let mut set = HashSet::with_capacity(9);
            set.insert(grid[i][j]);
            set.insert(grid[i + 1][j]);
            set.insert(grid[i + 2][j]);
            set.insert(grid[i][j + 1]);
            set.insert(grid[i + 1][j + 1]);
            set.insert(grid[i + 2][j + 1]);
            set.insert(grid[i][j + 2]);
            set.insert(grid[i + 1][j + 2]);
            set.insert(grid[i + 2][j + 2]);
            set = set.into_iter().filter(|&i| i > 0 && i < 10).collect();
            if set.len() != 9 || set.iter().sum::<i32>() != 45 {
                continue;
            }
            let sum = grid[i][j] + grid[i + 1][j] + grid[i + 2][j];
            println!("sum: {}", sum);
            if grid[i][j + 1] + grid[i + 1][j + 1] + grid[i + 2][j + 1] == sum &&
               grid[i][j + 2] + grid[i + 1][j + 2] + grid[i + 2][j + 2] == sum &&
               grid[i][j] + grid[i][j + 1] + grid[i][j + 2] == sum &&
               grid[i + 1][j] + grid[i + 1][j + 1] + grid[i + 1][j + 2] == sum &&
               grid[i + 2][j] + grid[i + 2][j + 1] + grid[i + 2][j + 2] == sum &&
               grid[i][j] + grid[i + 1][j + 1] + grid[i + 2][j + 2] == sum &&
               grid[i][j + 2] + grid[i + 1][j + 1] + grid[i + 2][j] == sum {
                ans += 1;
            }
        }
    }
    ans
}