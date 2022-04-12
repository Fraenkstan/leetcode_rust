mod topic_483;
mod topic_494;
mod topic_496;

#[cfg(test)]
mod tests {
    use crate::topic_483::smallest_good_base;
    use crate::topic_494::find_target_sum_ways;
    use crate::topic_496::next_greater_element;

    #[test]
    fn test() {
        println!("{}", 13_f64.powf(1.0 / 3.0));
    }

    #[test]
    fn topic_483() {
        println!("{}", smallest_good_base("13".to_string()));
        // println!("{}", smallest_good_base("1000000000000000000".to_string()));
    }

    #[test]
    fn topic_494() {
        let nums = vec![1, 1, 1, 1, 1];
        println!("{}", find_target_sum_ways(nums, 3));
    }

    #[test]
    fn topic_496() {
        println!(
            "{:?}",
            next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2])
        );
        println!("{:?}", next_greater_element(vec![2, 4], vec![1, 2, 3, 4]));
    }
}
