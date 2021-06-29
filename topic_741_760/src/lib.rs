
mod topic_752;

#[cfg(test)]
mod tests {

    use crate::topic_752::open_lock;

    #[test]
    fn solution_752() {
        println!("{}", open_lock(vec!["0201".to_string(), "0101".to_string(), "0102".to_string(),
                                      "1212".to_string(), "2002".to_string()], "0202".to_string()));
        println!("{}", open_lock(vec!["8888".to_string()], "0009".to_string()));
        println!("{}", open_lock(vec!["8887".to_string(), "8889".to_string(),
                                      "8878".to_string(), "8898".to_string(), "8788".to_string(),
                                      "8988".to_string(), "7888".to_string(), "9888".to_string()], "8888".to_string()));
        println!("{}", open_lock(vec!["0000".to_string()], "8888".to_string()));
    }
}
