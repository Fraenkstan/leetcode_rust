
mod topic_383;
mod topic_392;

#[cfg(test)]
mod tests {

    use crate::topic_383::can_construct;
    use crate::topic_392::is_subsequence;

    #[test]
    fn solution_383() {
        println!("{}", can_construct("a".to_string(), "b".to_string()));
        println!("{}", can_construct("aa".to_string(), "ab".to_string()));
        println!("{}", can_construct("aa".to_string(), "aab".to_string()));
    }

    #[test]
    fn solution_392() {
        println!("{}", is_subsequence("abd".to_string(), "ahbgdc".to_string()));
        println!("{}", is_subsequence("axc".to_string(), "ahbgdc".to_string()));
    }
}
