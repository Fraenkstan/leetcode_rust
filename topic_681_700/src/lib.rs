mod topic_682;
mod topic_684;
mod topic_688;
mod topic_693;
mod topic_697;

extern crate trace;

#[cfg(test)]
mod tests {

    use crate::topic_682::cal_points;
    use crate::topic_684::find_redundant_connection;
    use crate::topic_688::knight_probability;
    use crate::topic_693::has_alternating_bits;
    use crate::topic_697::find_shortest_sub_array;

    #[test]
    fn solution_682() {
        println!(
            "{}",
            cal_points(vec![
                "5".to_string(),
                "2".to_string(),
                "C".to_string(),
                "D".to_string(),
                "+".to_string()
            ])
        );
        println!(
            "{}",
            cal_points(vec![
                "5".to_string(),
                "-2".to_string(),
                "4".to_string(),
                "C".to_string(),
                "D".to_string(),
                "9".to_string(),
                "+".to_string(),
                "+".to_string()
            ])
        );
        println!("{}", cal_points(vec!["1".to_string()]));
    }

    #[test]
    fn solution_684() {
        let edges = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4], vec![1, 5]];
        println!("{:?}", find_redundant_connection(edges));
    }

    #[test]
    fn solution_688() {
        println!("{}", knight_probability(3, 2, 0, 0));
        println!("{}", knight_probability(1, 0, 0, 0));
    }

    #[test]
    fn solution_693() {
        println!("{}", has_alternating_bits(5));
        println!("{}", has_alternating_bits(7));
        println!("{}", has_alternating_bits(11));
    }

    #[test]
    fn solution_697() {
        let nums = vec![1, 2, 2, 3, 1, 4, 2];
        println!("{}", find_shortest_sub_array(nums));
        let nums = vec![1, 2, 2, 3, 1];
        println!("{}", find_shortest_sub_array(nums));
    }
}
