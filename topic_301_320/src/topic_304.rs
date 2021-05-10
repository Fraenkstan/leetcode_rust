
struct NumMatrix (Vec<Vec<i32>>);


/*
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {

    fn new(mut matrix: Vec<Vec<i32>>) -> Self {
        if matrix.len()==0{Self(vec![vec![0]])}else{
            matrix.iter_mut().for_each(|i|{i.iter_mut().fold(0,|s,x|{*x+=s;*x});});
            let s=vec![0;matrix[0].len()];
            matrix.iter_mut().fold(s,|s,x|{s.iter().zip(x.iter_mut()).for_each(|(s,x)|*x+=s);x.to_vec()});
            Self(matrix)}
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        self.0[row2 as usize][col2 as usize] - if col1 > 0 {
            self.0[row2 as usize][(col1-1)as usize]
        }
        else{0} - if row1 > 0{
            self.0[(row1-1)as usize][col2 as usize]
        }
        else{0} + if col1 * row1 > 0{
            self.0[(row1-1)as usize][(col1-1)as usize]
        }
        else{0}
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */
