
mod topic_572;
mod topic_575;
mod topic_576;

#[cfg(test)]
mod tests {
    use crate::topic_575::distribute_candies;
    use crate::topic_576::find_paths;

    #[test]
    fn solution_575() {
        println!("{}", distribute_candies(vec![1,1,2,2,3,3]));
        println!("{}", distribute_candies(vec![1,1,2,3]));
    }

    #[test]
    fn solution_576() {
        println!("{}", find_paths(2, 2, 2, 0, 0));
        println!("{}", find_paths(1, 3, 3, 0, 1));
    }
}
