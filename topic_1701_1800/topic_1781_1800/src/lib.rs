
mod topic_1791;

#[cfg(test)]
mod tests {
    use crate::topic_1791::find_center;

    #[test]
    fn solution_1791() {
        println!("{}", find_center(vec![vec![1,2], vec![2,3], vec![4,2]]));
        println!("{}", find_center(vec![vec![1,2], vec![5,1], vec![1,3], vec![1,4]]));
    }
}
