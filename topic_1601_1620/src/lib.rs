mod topic_1601;

#[cfg(test)]
mod tests {
    use crate::topic_1601::maximum_requests;

    #[test]
    fn solution_1601() {
        println!(
            "{}",
            maximum_requests(3, vec![vec![0, 0], vec![1, 2], vec![2, 1]])
        );
        println!(
            "{}",
            maximum_requests(4, vec![vec![0, 3], vec![3, 1], vec![1, 2], vec![2, 0]])
        );
    }
}
