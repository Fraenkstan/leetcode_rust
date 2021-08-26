
mod topic_551;
mod topic_552;
mod topic_553;
mod topic_554;
mod topic_556;
mod topic_557;
mod topic_560;

#[cfg(test)]
mod tests {
    use crate::topic_551::check_record;
    use crate::topic_552::check_record as check_record_2;
    use crate::topic_553::optimal_division;
    use crate::topic_554::least_bricks;
    use crate::topic_557::reverse_words;
    use crate::topic_560::subarray_sum;

    #[test]
    fn solution_551() {
        println!("{}", check_record("PPALLP".to_string()));
        println!("{}", check_record("PPALLL".to_string()));
    }

    #[test]
    fn solution_552() {
        println!("{}", check_record_2(2));
        println!("{}", check_record_2(1));
        println!("{}", check_record_2(10101));
    }

    #[test]
    fn solution_553() {
        println!("{}", optimal_division(vec![1000, 100, 10, 2]));
    }

    #[test]
    fn solution_554() {
        println!("{}", least_bricks(vec![vec![1], vec![1], vec![1]]));
        println!("{}", least_bricks(vec![vec![1,2,2,1], vec![3,1,2], vec![1,3,2],
                                         vec![2,4], vec![3,1,2], vec![1,3,1,1]]));
    }

    #[test]
    fn solution_557() {
        println!("{}", reverse_words("Let's take LeetCode contest".to_string()));
    }

    #[test]
    fn solution_560() {
        println!("{}", subarray_sum(vec![1,1,1], 2));
        println!("{}", subarray_sum(vec![-1,-1,1], 0));
    }
}
