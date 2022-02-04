mod topic_1688;

#[cfg(test)]
mod tests {
    use crate::topic_1688::number_of_matches;

    #[test]
    fn solution_1688() {
        println!("{}", number_of_matches(7));
        println!("{}", number_of_matches(14));
        println!("{}", number_of_matches(9));
    }
}
