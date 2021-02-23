
mod topic_1423;
mod topic_1438;

#[cfg(test)]
mod tests {

    use crate::topic_1423::max_score;
    use crate::topic_1438::longest_subarray;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn solution_1423() {
        let card_points = vec![1,2,3,4,5,6,1];
        println!("{}", max_score(card_points, 3));
        let card_points = vec![9,7,7,9,7,7,9];
        println!("{}", max_score(card_points, 7));
    }

    #[test]
    fn solution_1438() {
        let nums = vec![8, 4, 2, 7];
        println!("{}", longest_subarray(nums, 4));

        let nums = vec![10,1,2,4,7,2];
        println!("{}", longest_subarray(nums, 5));
    }
}
