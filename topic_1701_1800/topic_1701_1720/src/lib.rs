mod topic_1711;
mod topic_1713;

#[cfg(test)]
mod tests {

    use crate::topic_1711::count_pairs;

    #[test]
    fn solution_1711() {
        println!("{}", count_pairs(vec![1, 3, 5, 7, 9]));
        println!("{}", count_pairs(vec![1, 1, 1, 3, 3, 3, 7]));
        println!(
            "{}",
            count_pairs(vec![
                64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
                64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64
            ])
        );
    }
}
