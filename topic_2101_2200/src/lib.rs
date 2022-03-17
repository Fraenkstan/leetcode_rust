
mod topic_2104;

#[cfg(test)]
mod tests {

    use crate::topic_2104::sub_array_ranges;

    #[test]
    fn solution_2104() {
        println!("{}", sub_array_ranges(vec![1,2,3]));
        println!("{}", sub_array_ranges(vec![1,3,3]));
        println!("{}", sub_array_ranges(vec![4,-2,-3,4,1]));
    }
}
