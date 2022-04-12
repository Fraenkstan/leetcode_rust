mod topic_1380;

#[cfg(test)]
mod tests {
    use crate::topic_1380::lucky_numbers;

    #[test]
    fn solution_1380() {
        println!(
            "{:?}",
            lucky_numbers(vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]])
        );
        println!(
            "{:?}",
            lucky_numbers(vec![
                vec![1, 10, 4, 2],
                vec![9, 3, 8, 7],
                vec![15, 16, 17, 12]
            ])
        );
        println!("{:?}", lucky_numbers(vec![vec![7, 8], vec![1, 2]]));
    }
}
