pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut ans: Vec<Vec<i32>> = vec![];
    for i in 1..num_rows + 1 {
        let i = i as usize;
        let mut row = vec![1; i as usize];
        for j in 1..i - 1 {
            let j = j as usize;
            row[j] = ans[i - 2][j - 1] + ans[i - 2][j];
        }
        ans.push(row);
    }
    ans
}
