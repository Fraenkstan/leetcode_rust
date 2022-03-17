
mod topic_424;
mod topic_432;

#[cfg(test)]
mod tests {
    use crate::topic_424::character_replacement;
    use crate::topic_432::AllOne;

    #[test]
    fn solution_424() {
        let s = "ABAB".to_string();
        let k = 2;
        println!("{}", character_replacement(s, k));
    }

    #[test]
    fn solution_432() {
        let mut all_one = AllOne::new();
        all_one.inc("hello".to_string());
        all_one.inc("hello".to_string());
        println!("{}", all_one.get_max_key());
        println!("{}", all_one.get_min_key());
        all_one.inc("leet".to_string());
        println!("{}", all_one.get_max_key());
        println!("{}", all_one.get_min_key());
        all_one.dec("hello".to_string());
        println!("{}", all_one.get_max_key());
        println!("{}", all_one.get_min_key());
    }
}
