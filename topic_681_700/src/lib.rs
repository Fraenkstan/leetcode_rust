
mod topic_684;
mod topic_697;

#[cfg(test)]
mod tests {

    use crate::topic_684::find_redundant_connection;
    use crate::topic_697::find_shortest_sub_array;

    #[test]
    fn solution_684() {
        let edges = vec![vec![1, 2], vec![2,3], vec![3,4], vec![1,4], vec![1,5]];
        println!("{:?}", find_redundant_connection(edges));
    }

    #[test]
    fn solution_697() {
        let nums = vec![1,2,2,3,1,4,2];
        println!(find_shortest_sub_array(nums));
        let nums = vec![1, 2, 2, 3, 1];
        println!(find_shortest_sub_array(nums));
    }
}
