
mod topic_1584;

#[cfg(test)]
mod tests {

    use crate::topic_1584::{min_cost_connect_points, kruskal};

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
}
