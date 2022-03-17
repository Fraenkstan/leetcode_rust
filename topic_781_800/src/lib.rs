
mod topic_781;
mod topic_782;
mod topic_787;
mod topic_789;
mod topic_797;
mod topic_798;

#[cfg(test)]
mod tests {

    use crate::topic_781::num_rabbits;
    use crate::topic_782::moves_to_chessboard;
    use crate::topic_787::find_cheapest_price;
    use crate::topic_789::escape_ghosts;
    use crate::topic_797::all_paths_source_target;
    use crate::topic_798::best_rotation;

    #[test]
    fn solution_781() {
        println!("{}", num_rabbits(vec![1,1,2]));
        println!("{}", num_rabbits(vec![10,10,10]));
        println!("{}", num_rabbits(vec![]));
        println!("{}", num_rabbits(vec![1,0,1,0,0]));
        println!("{}", num_rabbits(vec![0,0,1,1,1,]));
        println!("{}", num_rabbits(vec![2,1,2,2,2,2,2,2,1,1]));
    }

    #[test]
    fn solution_782() {
        println!("{}", moves_to_chessboard(vec![vec![0,1,1,0], vec![0,1,1,0],
                                                vec![1,0,0,1], vec![1,0,0,1]]));
        println!("{}", moves_to_chessboard(vec![vec![0,1], vec![1,0]]));
        println!("{}", moves_to_chessboard(vec![vec![1,0], vec![1,0]]));
        println!("{}", moves_to_chessboard(vec![vec![1,1,0], vec![0,0,1], vec![0,0,1]]));
    }

    #[test]
    fn solution_787() {
        println!("{}", find_cheapest_price(3, vec![vec![0,1,100], vec![1,2,100],
                                                   vec![0,2,500]], 0, 2, 1));
        println!("{}", find_cheapest_price(3, vec![vec![0,1,100], vec![1,2,100],
                                                   vec![0,2,500]], 0, 2, 0));
    }

    #[test]
    fn solution_789() {
        println!("{}", escape_ghosts(vec![vec![1,0], vec![0,3]], vec![0,1]));
        println!("{}", escape_ghosts(vec![vec![1,0]], vec![2,0]));
        println!("{}", escape_ghosts(vec![vec![2,0]], vec![1,0]));
        println!("{}", escape_ghosts(vec![vec![5,0], vec![-10,-2], vec![0,-5],
                                          vec![-2,-2], vec![-7,1]], vec![7,7]));
        println!("{}", escape_ghosts(vec![vec![-1,0], vec![0,1], vec![-1,0],
                                          vec![0,1], vec![-1,0]], vec![0,0]));
    }

    #[test]
    fn solution_797() {
        println!("{:?}", all_paths_source_target(vec![vec![1,2], vec![3], vec![3], vec![]]));
        println!("{:?}", all_paths_source_target(vec![vec![4,3,1], vec![3,2,4], vec![3], vec![4], vec![]]));
        println!("{:?}", all_paths_source_target(vec![vec![1], vec![]]));
        println!("{:?}", all_paths_source_target(vec![vec![1,2,3], vec![2], vec![3], vec![]]));
    }

    #[test]
    fn solution_798() {
        println!("{}", best_rotation(vec![2,3,1,4,0]));
        println!("{}", best_rotation(vec![1,3,0,2,4]));
    }
}
