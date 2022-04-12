mod topic_62;
mod topic_63;
mod topic_64;
mod topic_65;
mod topic_70;

#[cfg(test)]
mod tests {
    use crate::topic_62::unique_paths;
    use crate::topic_63::unique_paths_with_obstacles;
    use crate::topic_64::min_path_sum;
    use crate::topic_65::is_number;
    use crate::topic_70::climb_stairs;

    #[test]
    fn solution_62() {
        println!("{}", unique_paths(3, 7));
        println!("{}", unique_paths(3, 2));
        println!("{}", unique_paths(7, 3));
        println!("{}", unique_paths(3, 3));
    }

    #[test]
    fn solution_63() {
        println!(
            "{}",
            unique_paths_with_obstacles(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]])
        );
        println!(
            "{}",
            unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]])
        );
        println!("{}", unique_paths_with_obstacles(vec![vec![0, 0]]));
    }

    #[test]
    fn solution_64() {
        println!(
            "{}",
            min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]])
        );
        println!("{}", min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]));
    }

    #[test]
    fn solution_65() {
        println!("{}", is_number("99e2.5".to_string()));
        println!("{}", is_number("53.5e93".to_string()));
        println!("{}", is_number("--6".to_string()));
        println!("{}", is_number("1E9".to_string()));
    }

    #[test]
    fn solution_70() {
        println!("{}", climb_stairs(2));
        println!("{}", climb_stairs(3));
    }
}
