
mod topic_1631;

#[cfg(test)]
mod tests {

    use crate::topic_1631::minimum_effort_path;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn solution_1631() {
        let heights = vec![vec![1,2,2], vec![3,8,2], vec![5,3,5]];
        let heights1 = vec![vec![1,2,1,1,1], vec![1,2,1,2,1],
                                          vec![1,2,1,2,1], vec![1,2,1,2,1],
                                          vec![1,1,1,2,1]];
        println!("{}", minimum_effort_path(heights));
        println!("{}", minimum_effort_path(heights1));
    }
}
