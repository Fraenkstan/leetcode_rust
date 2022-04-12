mod topic_721;
mod topic_726;
mod topic_728;

#[cfg(test)]
mod tests {

    use crate::topic_721::accounts_merge;
    use crate::topic_726::count_of_atoms;
    use crate::topic_728::Solution;

    #[test]
    fn solution_721() {
        let accounts = vec![
            vec![
                "John".to_string(),
                "johnsmith@mail.com".to_string(),
                "john00@mail.com".to_string(),
            ],
            vec!["John".to_string(), "johnnybravo@mail.com".to_string()],
            vec![
                "John".to_string(),
                "johnsmith@mail.com".to_string(),
                "john_newyork@mail.com".to_string(),
            ],
            vec!["Mary".to_string(), "mary@mail.com".to_string()],
        ];
        println!("{:?}", accounts_merge(accounts))
    }

    #[test]
    fn solution_726() {
        println!("{}", count_of_atoms("H2O".to_string()));
        println!("{}", count_of_atoms("Mg(OH)2".to_string()));
        println!("{}", count_of_atoms("K4(ON(SO3)2)2".to_string()));
    }

    #[test]
    fn solution_728() {
        println!("{:?}", Solution::self_dividing_numbers(1, 22));
        println!("{:?}", Solution::self_dividing_numbers(47, 85));
    }
}
