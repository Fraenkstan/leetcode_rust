mod topic_523;
mod topic_525;

#[cfg(test)]
mod tests {
    use crate::topic_523::check_subarray_sum;
    use crate::topic_525::find_max_length;

    #[test]
    fn topic_523() {
        let nums = vec![23, 2, 6, 4, 7];
        let k = 13;
        println!("{}", check_subarray_sum(nums, k));
    }

    #[test]
    fn topic_525() {
        let nums = vec![0,1,0];
        println!("{}", find_max_length(nums));
    }
}
