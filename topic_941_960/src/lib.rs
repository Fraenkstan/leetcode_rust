
mod topic_947;

#[cfg(test)]
mod tests {

    use crate::topic_947::remove_stones;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn solution_947() {
        let stones = vec![vec![0,0], vec![0,1], vec![1,0],
                                         vec![1,2], vec![2,1], vec![2,2]];
        println!("{}", remove_stones(stones));
    }
}
