
mod topic_1584;
mod topic_1600;

#[cfg(test)]
mod tests {

    use crate::topic_1584::{min_cost_connect_points, kruskal};
    use crate::topic_1600::ThroneInheritance;

    #[test]
    fn it_works() {
        for i in 0..10 {
            println!("{}", i);
        }
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn solution_1584() {
        let points = vec![vec![0, 0], vec![2, 2], vec![3, 10],
                                         vec![5, 2], vec![7, 0]];
        println!("{}", kruskal(points.clone()));

        //TODO: 优化版的kruskal写法失败🤢🤢🤢🤢
        println!("{}", min_cost_connect_points(points));
    }

    #[test]
    fn solution_1600() {
        let mut root = ThroneInheritance::new("king".to_string());
        root.birth("king".to_string(), "andy".to_string());
        root.birth("king".to_string(), "bod".to_string());
    }
}
