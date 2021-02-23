
mod topic_480;

#[cfg(test)]
mod tests {
    use crate::topic_480::median_sliding_window;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn solution_480() {
        let nums = vec![1,3,-1,-3,5,3,6,7];
        println!("{:?}", median_sliding_window(nums, 3))
    }
}
