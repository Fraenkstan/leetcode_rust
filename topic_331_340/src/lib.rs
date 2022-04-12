mod topic_334;
mod topic_338;

#[cfg(test)]
mod tests {
    use crate::topic_334::increasing_triplet;
    use crate::topic_338::count_bits;

    #[test]
    fn solution_334() {
        println!("{}", increasing_triplet(vec![1, 2, 3, 4, 5]));
        println!("{}", increasing_triplet(vec![5, 4, 3, 2, 1]));
        println!("{}", increasing_triplet(vec![2, 1, 5, 0, 4, 6]));
    }

    #[test]
    fn solution_338() {
        println!("{:?}", count_bits(5));
        println!("{:?}", count_bits(9));
    }
}
