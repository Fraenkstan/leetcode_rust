
mod topic_392;

#[cfg(test)]
mod tests {

    use crate::topic_392::is_subsequence;

    #[test]
    fn solution() {
        println!("{}", is_subsequence("abd".to_string(), "ahbgdc".to_string()));
        println!("{}", is_subsequence("axc".to_string(), "ahbgdc".to_string()));
    }
}
