
mod topic_888;

#[cfg(test)]
mod tests {
    use crate::topic_888::fair_candy_swap;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn solution_888() {
        let alice = vec![1, 2, 5];
        let bob = vec![2, 4];
        println!("{:?}", fair_candy_swap(alice, bob))
    }
}
