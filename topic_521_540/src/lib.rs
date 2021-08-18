
mod topic_523;
mod topic_525;
mod topic_526;

#[cfg(test)]
mod tests {
    use crate::topic_523::check_subarray_sum;
    use crate::topic_525::find_max_length;
    use crate::topic_526::{count_arrangement, count_arrangement_backstrace};

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

    #[test]
    fn topic_526() {
        for i in 1usize..16 {
            println!("count_ones({}): {}", i, i32::count_ones(i as i32));
            println!("dp result: {}", count_arrangement(i as i32));
            println!("backstrace result: {}", count_arrangement_backstrace(i as i32));
        }
    }
}
