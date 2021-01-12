mod topic_228;
mod topic_239;

#[cfg(test)]
mod tests {

    use crate::topic_228::summary_ranges;
    use crate::topic_239::max_sliding_window;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn solution_228() {
        println!("Hello, world!");
        let nums = vec![0,1,2,4,5,7];
        let result = summary_ranges(nums);
        for iter in result {
            println!("{}", iter);
        }
    }

    #[test]
    fn solution_239() {
        println!("Hello, world!");
        let nums = vec![1,3,-1,-3,5,3,6,7];
        let results = max_sliding_window(nums, 3);
        println!("{:?}", results);
    }
}
