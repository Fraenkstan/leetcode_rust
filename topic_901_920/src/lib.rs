mod topic_917;

#[cfg(test)]
mod tests {

    use crate::topic_917::reverse_only_letters;

    #[test]
    fn solution_917() {
        println!("{}", reverse_only_letters("ab-cd".to_string()));
        println!("{}", reverse_only_letters("a-bC-dEf-ghIj".to_string()));
        println!(
            "{}",
            reverse_only_letters("Test1ng-Leet=code-Q!".to_string())
        );
    }
}
