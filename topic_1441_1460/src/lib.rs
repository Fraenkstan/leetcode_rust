
mod topic_1449;

#[cfg(test)]
mod tests {
    use crate::topic_1449::largest_number;

    #[test]
    fn solution_1449() {
        println!("{}", largest_number(vec![4,3,2,5,6,7,2,5,5], 9));
    }
}
