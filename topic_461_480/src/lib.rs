
mod topic_474;
mod topic_480;

#[cfg(test)]
mod tests {
    use crate::topic_474::find_max_form;
    use crate::topic_480::median_sliding_window;

    #[test]
    fn solution_474() {
        println!("{}", find_max_form(vec!["10".to_string(), "0001".to_string(),
                                          "111001".to_string(), "1".to_string(), "0".to_string()],
                                     5, 3));
        println!("{}", find_max_form(vec!["10".to_string(), "0".to_string(), "1".to_string()],
                                     1, 1));
    }

    #[test]
    fn solution_480() {
        let nums = vec![1,3,-1,-3,5,3,6,7];
        println!("{:?}", median_sliding_window(nums, 3))
    }
}
