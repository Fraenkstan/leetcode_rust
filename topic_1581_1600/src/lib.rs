mod topic_1583;
mod topic_1584;
mod topic_1588;
mod topic_1600;

#[cfg(test)]
mod tests {

    use crate::topic_1583::unhappy_friends;
    use crate::topic_1584::{kruskal, min_cost_connect_points};
    use crate::topic_1588::sum_odd_length_subarrays;
    use crate::topic_1600::ThroneInheritance;

    #[test]
    fn solution_1583() {
        println!(
            "{}",
            unhappy_friends(
                4,
                vec![vec![1, 2, 3], vec![3, 2, 0], vec![3, 1, 0], vec![1, 2, 0]],
                vec![vec![0, 1], vec![2, 3]]
            )
        );
        println!(
            "{}",
            unhappy_friends(2, vec![vec![1], vec![0]], vec![vec![1, 0]])
        );
        println!(
            "{}",
            unhappy_friends(
                4,
                vec![vec![1, 3, 2], vec![2, 3, 0], vec![1, 3, 0], vec![0, 2, 1]],
                vec![vec![1, 3], vec![0, 2]]
            )
        );
    }

    #[test]
    fn solution_1584() {
        let points = vec![vec![0, 0], vec![2, 2], vec![3, 10], vec![5, 2], vec![7, 0]];
        println!("{}", kruskal(points.clone()));

        //TODO: 优化版的kruskal写法失败🤢🤢🤢🤢
        println!("{}", min_cost_connect_points(points));
    }

    #[test]
    fn solution_1588() {
        println!("{}", sum_odd_length_subarrays(vec![1, 4, 2, 5, 3]));
    }

    #[test]
    fn solution_1600() {
        let mut root = ThroneInheritance::new("king".to_string());
        root.birth("king".to_string(), "andy".to_string());
        root.birth("king".to_string(), "bod".to_string());
    }
}
