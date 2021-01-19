
mod topic_721;

#[cfg(test)]
mod tests {

    use crate::topic_721::accounts_merge;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn solution_721() {
        let accounts =
            vec![vec!["John".to_string(), "johnsmith@mail.com".to_string(), "john00@mail.com".to_string()],
                 vec!["John".to_string(), "johnnybravo@mail.com".to_string()],
                 vec!["John".to_string(), "johnsmith@mail.com".to_string(), "john_newyork@mail.com".to_string()],
                 vec!["Mary".to_string(), "mary@mail.com".to_string()]];
        println!("{:?}", accounts_merge(accounts))
    }
}
