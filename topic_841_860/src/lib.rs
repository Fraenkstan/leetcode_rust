mod topic_847;
mod topic_852;

#[cfg(test)]
mod tests {

    use crate::topic_847::shortest_path_length;
    use crate::topic_852::peak_index_in_mountain_array;

    #[test]
    fn solution_847() {
        println!(
            "{}",
            shortest_path_length(vec![vec![1, 2, 3], vec![0], vec![0], vec![0]])
        );
        println!(
            "{}",
            shortest_path_length(vec![
                vec![1],
                vec![0, 2, 4],
                vec![1, 3, 4],
                vec![2],
                vec![1, 2]
            ])
        );
    }

    #[test]
    fn solution_852() {
        println!("{}", peak_index_in_mountain_array(vec![0, 1, 0]));
        println!("{}", peak_index_in_mountain_array(vec![0, 2, 1, 0]));
        println!(
            "{}",
            peak_index_in_mountain_array(vec![24, 69, 100, 99, 79, 78, 67, 36, 26, 19])
        );
    }
}
