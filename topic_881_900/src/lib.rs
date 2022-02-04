
mod topic_881;
mod topic_884;
mod topic_888;
mod topic_897;

#[cfg(test)]
mod tests {
    use crate::topic_881::num_rescue_boats;
    use crate::topic_888::fair_candy_swap;

    #[test]
    fn solution_881() {
        println!("{}", num_rescue_boats(vec![1,2], 3));
        println!("{}", num_rescue_boats(vec![3,2,2,1], 3));
        println!("{}", num_rescue_boats(vec![3,5,3,4], 5));
    }

    #[test]
    fn solution_888() {
        let alice = vec![1, 2, 5];
        let bob = vec![2, 4];
        println!("{:?}", fair_candy_swap(alice, bob))
    }
}
