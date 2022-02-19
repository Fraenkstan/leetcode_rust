extern crate rand;

mod topic_523;
mod topic_524;
mod topic_525;
mod topic_526;
mod topic_528;
mod topic_540;

#[cfg(test)]
mod tests {
    use crate::topic_523::check_subarray_sum;
    use crate::topic_524::find_longest_word;
    use crate::topic_525::find_max_length;
    use crate::topic_526::{count_arrangement, count_arrangement_backstrace};
    use crate::topic_528::Solution;
    use crate::topic_540::single_non_duplicate;

    #[test]
    fn topic_523() {
        let nums = vec![23, 2, 6, 4, 7];
        let k = 13;
        println!("{}", check_subarray_sum(nums, k));
    }

    #[test]
    fn solution_524() {
        let dictionary = vec!["ale".to_string(), "apple".to_string(),
                              "monkey".to_string(), "plea".to_string()];
        println!("{}", find_longest_word("abpcplea".to_string(), dictionary));
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

    #[test]
    fn solution_528() {
        let test = Solution::new(vec![1]);
        println!("{}", test.pick_index());
        let test1 = Solution::new(vec![1,3]);
        println!("{}", test1.pick_index());
        println!("{}", test1.pick_index());
        println!("{}", test1.pick_index());
        println!("{}", test1.pick_index());
        println!("{}", test1.pick_index());
    }

    #[test]
    fn solution_540() {
        println!("{}", single_non_duplicate(vec![1,1,2,3,3,4,4,8,8]));
        println!("{}", single_non_duplicate(vec![3,3,7,7,10,11,11]));
    }
}
