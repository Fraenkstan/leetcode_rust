
mod topic_665;

#[cfg(test)]
mod tests {
    use crate::topic_665::check_possibility;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn solution_665() {
        let nums = vec![3, 4, 2, 5];
        println!("{}", check_possibility(nums));
        let nums = vec![5, 7, 1, 8];
        println!("{}", check_possibility(nums));
    }
}
