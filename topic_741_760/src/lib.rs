mod topic_743;
mod topic_744;
mod topic_752;

#[cfg(test)]
mod tests {

    use crate::topic_743::network_delay_time;
    use crate::topic_744::next_greatest_letter;
    use crate::topic_752::open_lock;

    #[test]
    fn solution_743() {
        println!(
            "{}",
            network_delay_time(vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]], 4, 2)
        );
        println!("{}", network_delay_time(vec![vec![1, 2, 1]], 2, 1));
        println!("{}", network_delay_time(vec![vec![1, 2, 1]], 2, 2));
    }

    #[test]
    fn solution_744() {
        println!("{}", next_greatest_letter(vec!['c', 'f', 'j'], 'a'));
        println!("{}", next_greatest_letter(vec!['c', 'f', 'j'], 'c'));
        println!("{}", next_greatest_letter(vec!['c', 'f', 'j'], 'd'));
        println!("{}", next_greatest_letter(vec!['a', 'b'], 'z'));
    }

    #[test]
    fn solution_752() {
        println!(
            "{}",
            open_lock(
                vec![
                    "0201".to_string(),
                    "0101".to_string(),
                    "0102".to_string(),
                    "1212".to_string(),
                    "2002".to_string()
                ],
                "0202".to_string()
            )
        );
        println!(
            "{}",
            open_lock(vec!["8888".to_string()], "0009".to_string())
        );
        println!(
            "{}",
            open_lock(
                vec![
                    "8887".to_string(),
                    "8889".to_string(),
                    "8878".to_string(),
                    "8898".to_string(),
                    "8788".to_string(),
                    "8988".to_string(),
                    "7888".to_string(),
                    "9888".to_string()
                ],
                "8888".to_string()
            )
        );
        println!(
            "{}",
            open_lock(vec!["0000".to_string()], "8888".to_string())
        );
    }
}
