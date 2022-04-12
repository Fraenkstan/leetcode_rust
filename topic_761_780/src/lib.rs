mod topic_762;
mod topic_766;
mod topic_773;
mod topic_778;
mod topic_780;

#[cfg(test)]
mod tests {

    use crate::topic_762::count_prime_set_bits;
    use crate::topic_766::is_toeplitz_matrix;
    use crate::topic_773::sliding_puzzle;
    use crate::topic_778::swim_in_water;
    use crate::topic_780::reaching_points;

    #[test]
    fn solution_762() {
        println!("{}", count_prime_set_bits(6, 10));
        println!("{}", count_prime_set_bits(10, 15));
    }

    #[test]
    fn solution_766() {
        let matrix1 = vec![vec![1, 2, 3, 4], vec![5, 1, 2, 3], vec![9, 5, 1, 2]];
        let matrix2 = vec![vec![1, 2], vec![2, 2]];
        println!("{}", is_toeplitz_matrix(matrix1));
        println!("{}", is_toeplitz_matrix(matrix2));
    }

    #[test]
    fn solution_773() {
        println!("{}", sliding_puzzle(vec![vec![1,2,3], vec![4,0,5]]));
        println!("{}", sliding_puzzle(vec![vec![1,2,3], vec![5,4,0]]));
        println!("{}", sliding_puzzle(vec![vec![4,1,2], vec![5,0,3]]));
    }

    #[test]
    fn solution_778() {
        // let grid = vec![vec![0, 2], vec![1, 3]];

        let grid = vec![
            vec![0, 1, 2, 3, 4],
            vec![24, 23, 22, 21, 5],
            vec![12, 13, 14, 15, 16],
            vec![11, 17, 18, 19, 20],
            vec![10, 9, 8, 7, 6],
        ];

        println!("{}", swim_in_water(grid));
    }

    #[test]
    fn solution_780() {
        println!("{}", reaching_points(1, 1, 3, 5));
        println!("{}", reaching_points(1, 1, 2, 2));
        println!("{}", reaching_points(1, 1, 1, 1));
    }
}
