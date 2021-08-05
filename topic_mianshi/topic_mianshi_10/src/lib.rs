
mod topic_10_02;

#[cfg(test)]
mod tests {

    use crate::topic_10_02::group_anagrams;

    #[test]
    fn solution_10_02() {
        println!("{:?}", group_anagrams(vec!["eat".to_string(), "tea".to_string(), "tan".to_string(),
                                                  "ate".to_string(), "nat".to_string(), "bat".to_string()]));
    }
}
