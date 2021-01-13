mod topic_684;

#[cfg(test)]
mod tests {

    use crate::topic_684::find_redundant_connection;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn solution_684() {
        let edges = vec![vec![1, 2], vec![2,3], vec![3,4], vec![1,4], vec![1,5]];
        println!("{:?}", find_redundant_connection(edges));
    }
}
