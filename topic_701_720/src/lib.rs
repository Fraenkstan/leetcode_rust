
mod topic_703;
mod topic_704;

#[cfg(test)]
mod tests {

    use crate::topic_704::search;

    #[test]
    fn solution_704() {
        println!("{}", search(vec![-1,0,3,5,9,12], 9));
        println!("{}", search(vec![-1,0,3,5,9,12], 2));
        println!("{}", search(vec![0,5], 5));
        println!("{}", search(vec![5], 5));
    }
}
