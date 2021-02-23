
mod topic_424;

#[cfg(test)]
mod tests {
    use crate::topic_424::character_replacement;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn solution_424() {
        let s = "ABAB".to_string();
        let k = 2;
        println!("{}", character_replacement(s, k));
    }
}
