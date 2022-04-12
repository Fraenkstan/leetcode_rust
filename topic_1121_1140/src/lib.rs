mod topic_1122;
mod topic_1137;
mod topic_1138;

#[cfg(test)]
mod tests {
    use crate::topic_1122::relative_sort_array;
    use crate::topic_1137::tribonacci;
    use crate::topic_1138::find_num_of_valid_words;

    #[test]
    fn solution_1122() {
        println!(
            "{:?}",
            relative_sort_array(
                vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
                vec![2, 1, 4, 3, 9, 6]
            )
        );
    }

    #[test]
    fn solution_1137() {
        println!("{}", tribonacci(4));
        println!("{}", tribonacci(25));
    }

    #[test]
    fn solution_1138() {
        let words = vec![
            "aaaa".to_string(),
            "asas".to_string(),
            "able".to_string(),
            "ability".to_string(),
            "actt".to_string(),
            "actor".to_string(),
            "access".to_string(),
        ];
        let puzzles = vec![
            "aboveyz".to_string(),
            "abrodyz".to_string(),
            "abslute".to_string(),
            "absoryz".to_string(),
            "actresz".to_string(),
            "gaswxyz".to_string(),
        ];
        println!("{:?}", find_num_of_valid_words(words, puzzles))
    }
}
