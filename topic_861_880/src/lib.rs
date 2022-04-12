mod topic_863;
mod topic_867;
mod topic_872;
mod topic_877;
mod topic_879;

#[cfg(test)]
mod tests {
    use crate::topic_867::transpose;
    use crate::topic_877::stone_game;
    use crate::topic_879::profitable_schemes;

    #[test]
    fn solution_867() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        println!("{:?}", transpose(matrix));
    }

    #[test]
    fn solution_877() {
        println!("{}", stone_game(vec![5, 3, 4, 5]));
    }

    #[test]
    fn solution_879() {
        println!("{}", profitable_schemes(5, 3, vec![2, 2], vec![2, 3]));
    }
}
