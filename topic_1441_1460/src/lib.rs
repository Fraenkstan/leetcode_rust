
mod topic_1447;
mod topic_1449;

#[cfg(test)]
mod tests {
    use crate::topic_1447::simplified_fractions;
    use crate::topic_1449::largest_number;

    #[test]
    fn solution_1447() {
        println!("{:?}", simplified_fractions(2));
        println!("{:?}", simplified_fractions(3));
        println!("{:?}", simplified_fractions(4));
    }

    #[test]
    fn solution_1449() {
        println!("{}", largest_number(vec![4,3,2,5,6,7,2,5,5], 9));
    }
}
