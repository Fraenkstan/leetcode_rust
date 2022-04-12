mod topic_1743;
mod topic_1748;

#[cfg(test)]
mod tests {
    use crate::topic_1743::restore_array;
    use crate::topic_1748::sum_of_unique;

    #[test]
    fn solution_1743() {
        println!(
            "{:?}",
            restore_array(vec![vec![2, 1], vec![3, 4], vec![3, 2]])
        );
        println!(
            "{:?}",
            restore_array(vec![vec![4, -2], vec![1, 4], vec![-3, 1]])
        );
        println!("{:?}", restore_array(vec![vec![100000, -100000]]));
    }

    #[test]
    fn solution_1748() {
        println!("{}", sum_of_unique(vec![1, 2, 3, 2]));
        println!("{}", sum_of_unique(vec![1, 1, 1, 1, 1]));
        println!("{}", sum_of_unique(vec![1, 2, 3, 4, 5]));
    }
}
