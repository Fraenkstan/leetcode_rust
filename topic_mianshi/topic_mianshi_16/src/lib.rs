
mod topic_16_01;
mod topic_16_02;
mod topic_16_03;

#[cfg(test)]
mod tests {
    use crate::topic_16_01::swap_numbers;
    use crate::topic_16_02::WordsFrequency;

    #[test]
    fn solution_16_01() {
        println!("{:?}", swap_numbers(vec![1,2]));
        println!("{:?}", swap_numbers(vec![i32::MAX, i32::MIN]));
    }

    #[test]
    fn solution_16_02() {
        let words = WordsFrequency::new(vec!["i".to_string(), "have".to_string(),
                                             "an".to_string(), "apple".to_string(), "he".to_string(),
                                             "have".to_string(), "a".to_string(), "pen".to_string()]);
        println!("{}", words.get("you".to_string()));
        println!("{}", words.get("have".to_string()));
        println!("{}", words.get("an".to_string()));
        println!("{}", words.get("apple".to_string()));
        println!("{}", words.get("pen".to_string()));
    }
}
