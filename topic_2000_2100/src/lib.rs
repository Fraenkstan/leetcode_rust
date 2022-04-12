mod topic_2006;
mod topic_2013;
mod topic_2016;
mod topic_2024;
mod topic_2043;
mod topic_2044;
mod topic_2047;
mod topic_2049;
mod topic_2055;

#[cfg(test)]
mod tests {

    use crate::topic_2006::count_k_difference;
    use crate::topic_2013::DetectSquares;
    use crate::topic_2016::maximum_difference;
    use crate::topic_2024::max_consecutive_answers;
    use crate::topic_2043::Bank;
    use crate::topic_2044::count_max_or_subsets;
    use crate::topic_2047::count_valid_words;
    use crate::topic_2049::count_highest_score_nodes;
    use crate::topic_2055::plates_between_candles;

    #[test]
    fn solution_2006() {
        println!("{}", count_k_difference(vec![1, 2, 2, 1], 1));
        println!("{}", count_k_difference(vec![1, 3], 3));
        println!("{}", count_k_difference(vec![3, 2, 1, 5, 4], 2));
    }

    #[test]
    fn solution_2013() {
        let mut data = DetectSquares::new();
        data.add(vec![3, 10]);
        data.add(vec![11, 2]);
        data.add(vec![3, 2]);
        println!("{}", data.count(vec![11, 10]));
        println!("{}", data.count(vec![14, 8]));
        data.add(vec![11, 2]);
        println!("{}", data.count(vec![11, 10]));
    }

    #[test]
    fn solution_2016() {
        println!("{}", maximum_difference(vec![7, 1, 5, 4]));
        println!("{}", maximum_difference(vec![9, 4, 3, 2]));
        println!("{}", maximum_difference(vec![1, 5, 2, 10]));
        println!("{}", maximum_difference(vec![5, 3, 1]));
    }

    #[test]
    fn solution_2024() {
        println!("{}", max_consecutive_answers("TTFF".to_string(), 2));
        println!("{}", max_consecutive_answers("TFFT".to_string(), 1));
        println!("{}", max_consecutive_answers("TTFTTFTT".to_string(), 1));
    }

    #[test]
    fn solution_2043() {
        let mut bank = Bank::new(vec![10, 100, 20, 50, 30]);
        println!("{}", bank.withdraw(3, 10));
        println!("{}", bank.transfer(5, 1, 20));
        println!("{}", bank.deposit(5, 20));
        println!("{}", bank.transfer(3, 4, 15));
        println!("{}", bank.withdraw(10, 50));
    }

    #[test]
    fn solution_2044() {
        println!("{}", count_max_or_subsets(vec![3, 1]));
        println!("{}", count_max_or_subsets(vec![2, 2, 2]));
        println!("{}", count_max_or_subsets(vec![3, 2, 1, 5]));
    }

    #[test]
    fn solution_2047() {
        println!("{}", count_valid_words(String::from("cat and  dog")));
        println!("{}", count_valid_words(String::from("!this  1-s b8d!")));
        println!(
            "{}",
            count_valid_words(String::from("alice and  bob are playing stone-game10"))
        );
        println!(
            "{}",
            count_valid_words(String::from(
                "he bought 2 pencils, 3 erasers, and 1  pencil-sharpener."
            ))
        );
    }

    #[test]
    fn solution_2049() {
        println!("{}", count_highest_score_nodes(vec![-1, 2, 0, 2, 0]));
        println!("{}", count_highest_score_nodes(vec![-1, 2, 0]));
    }

    #[test]
    fn solution_2055() {
        println!(
            "{:?}",
            plates_between_candles("**|**|***|".to_string(), vec![vec![2, 5], vec![5, 9]])
        );
        println!(
            "{:?}",
            plates_between_candles(
                "***|**|*****|**||**|*".to_string(),
                vec![vec![1, 17], vec![4, 5], vec![14, 17], vec![5, 11]]
            )
        );
    }
}
