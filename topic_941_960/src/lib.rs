mod topic_947;
mod topic_954;

#[cfg(test)]
mod tests {

    use crate::topic_947::remove_stones;
    use crate::topic_954::can_reorder_doubled;

    #[test]
    fn solution_947() {
        let stones = vec![
            vec![0, 0],
            vec![0, 1],
            vec![1, 0],
            vec![1, 2],
            vec![2, 1],
            vec![2, 2],
        ];
        println!("{}", remove_stones(stones));
    }

    #[test]
    fn solution_954() {
        println!("{}", can_reorder_doubled(vec![3, 1, 3, 6]));
        println!("{}", can_reorder_doubled(vec![2, 1, 2, 6]));
        println!("{}", can_reorder_doubled(vec![4, -2, 2, -4]));
        println!("{}", can_reorder_doubled(vec![2, 1, 2, 1, 1, 1, 2, 2]));
    }
}
