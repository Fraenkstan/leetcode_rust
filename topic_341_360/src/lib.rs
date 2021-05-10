
mod topic_354;

#[cfg(test)]
mod tests {

    use crate::topic_354::max_envelopes;

    #[test]
    fn solution_354() {
        println!("{}", max_envelopes(vec![vec![5,4],vec![6,4],vec![6,7],vec![2,3]]));
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
