
mod topic_703;
mod topic_704;
mod topic_720;

#[cfg(test)]
mod tests {

    use crate::topic_704::search;
    use crate::topic_720::longest_word;

    #[test]
    fn solution_704() {
        println!("{}", search(vec![-1,0,3,5,9,12], 9));
        println!("{}", search(vec![-1,0,3,5,9,12], 2));
        println!("{}", search(vec![0,5], 5));
        println!("{}", search(vec![5], 5));
    }

    #[test]
    fn solution_720() {
        println!("{}", longest_word(vec!["w".to_string(), "wo".to_string(), "wor".to_string(),
                                         "worl".to_string(), "world".to_string()]));
        println!("{}", longest_word(vec!["a".to_string(), "banana".to_string(), "app".to_string(),
                                         "appl".to_string(), "ap".to_string(), "apply".to_string(),
                                         "apple".to_string()]));
    }
}
