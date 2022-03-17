
mod topic_564;

#[cfg(test)]
mod tests {
    use crate::topic_564::nearest_palindromic;

    #[test]
    fn solution_564() {
        println!("{}", nearest_palindromic("123".to_string()));
        println!("{}", nearest_palindromic("1".to_string()));
    }
}
