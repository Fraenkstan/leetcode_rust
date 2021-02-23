

pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
    let col_len = matrix[0].len();
    let mut last_row = &matrix[0][0..col_len - 1];
    for next_row in matrix.iter().skip(1) {
        if last_row != &next_row[1..] {
            return false;
        }
        else {
            last_row = &next_row[0..col_len - 1];
        }
    }
    true
}

pub trait Toeplitz {

    fn is_toeplitz_matrix<'a, E, I, J>(matrix: &'a J) -> bool
        where
            &'a E: Eq,
            &'a I: IntoIterator<Item = &'a E>,
            &'a J: IntoIterator<Item = &'a I>,
            E: 'a,
            I: 'a,
    {
        Iterator::zip(matrix.into_iter(), matrix.into_iter().skip(1))
            .all(|(r, s)|
                Iterator::zip(r.into_iter(), s.into_iter().skip(1)).all(|(m, n)| m == n))
    }
}