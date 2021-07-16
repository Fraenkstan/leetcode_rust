
mod topic_17_10;

#[cfg(test)]
mod tests {
    use crate::topic_17_10::majority_element;

    #[test]
    fn solution_17_10() {
        println!("{}", majority_element(vec![1,2,5,9,5,9,5,5,5]));
        println!("{}", majority_element(vec![3,2]));
        println!("{}", majority_element(vec![2,2,1,1,1,2,]));
    }
}
