
mod topic_483;
mod topic_494;

#[cfg(test)]
mod tests {
    use crate::topic_483::smallest_good_base;
    use crate::topic_494::find_target_sum_ways;

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
}
