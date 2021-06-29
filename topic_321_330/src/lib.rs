
mod topic_322;

#[cfg(test)]
mod tests {
    use crate::topic_322::coin_change;

    #[test]
    fn solution_322() {
        println!("{}", coin_change(vec![1, 2, 5], 11));
        println!("{}", coin_change(vec![1], 0));
    }
}
