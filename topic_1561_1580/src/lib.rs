mod topic_1579;

#[cfg(test)]
mod tests {
    use crate::topic_1579::max_num_edges_to_remove;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn solution_1579() {
        let edges = vec![
            vec![3, 1, 2],
            vec![3, 2, 3],
            vec![1, 1, 3],
            vec![1, 2, 4],
            vec![1, 1, 2],
            vec![2, 3, 4],
        ];
        println!("{}", max_num_edges_to_remove(4, edges));

        let edges = vec![vec![3, 1, 2], vec![3, 2, 3], vec![1, 1, 4], vec![2, 1, 4]];
        println!("{}", max_num_edges_to_remove(4, edges));

        let edges = vec![vec![3, 2, 3], vec![1, 1, 2], vec![2, 3, 4]];
        println!("{}", max_num_edges_to_remove(4, edges));
    }
}
