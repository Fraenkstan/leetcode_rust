
mod topic_26;
mod topic_39;

#[cfg(test)]
mod tests {

    use crate::topic_26::remove_duplicates;
    use crate::topic_39::combination_sum;

    #[test]
    fn solution_26() {
        println!("{}", remove_duplicates(&mut vec![1, 1, 2]));
        println!("{}", remove_duplicates(&mut vec![0,0,1,1,1,2,2,3,3,4]));
    }

    #[test]
    fn solution_39() {
        println!("{:?}", combination_sum(vec![2,3,6,7], 7));
        println!("{:?}", combination_sum(vec![2,3,5], 8));
    }
}
