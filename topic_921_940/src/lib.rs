mod topic_930;

#[cfg(test)]
mod tests {
    use crate::topic_930::num_subarrays_with_sum;

    #[test]
    fn solution_930() {
        println!("{}", num_subarrays_with_sum(vec![1, 0, 1, 0, 1], 2));
        println!("{}", num_subarrays_with_sum(vec![0, 0, 0, 0, 0], 0));
    }
}
