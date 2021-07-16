
mod topic_offer_53;

#[cfg(test)]
mod tests {

    use crate::topic_offer_53::search;

    #[test]
    fn solution_53() {
        println!("{}", search(vec![5,7,7,8,8,10], 8));
        println!("{}", search(vec![5,7,7,8,8,10], 6));
        println!("{}", search(vec![1], 1));
        println!("{}", search(vec![2,2], 2));
        println!("{}", search(vec![1,4], 4));
        println!("{}", search(vec![5,7,7,8,8,10], 6));
    }
}
