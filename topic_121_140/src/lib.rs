mod topic_121;

#[cfg(test)]
mod tests {

    use crate::topic_121::max_profit;

    #[test]
    fn solution_121() {
        println!("{}", max_profit(vec![7, 1, 5, 3, 6, 4]));
        println!("{}", max_profit(vec![7, 6, 4, 3, 1]));
    }
}
