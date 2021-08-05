
mod topic_581;
mod topic_583;

#[cfg(test)]
mod tests {

    use crate::topic_581::find_unsorted_subarray;
    use crate::topic_583::min_distance;

    #[test]
    fn solution_581() {
        println!("{}", find_unsorted_subarray(vec![2,6,4,8,10,9,15]));
        println!("{}", find_unsorted_subarray(vec![1,2,3,4]));
        println!("{}", find_unsorted_subarray(vec![1]));
        println!("{}", find_unsorted_subarray(vec![1,3,2,3,3]));
        println!("{}", find_unsorted_subarray(vec![2,3,3,2,4]));
        println!("{}", find_unsorted_subarray(vec![1,3,5,4,2]));
        println!("{}", find_unsorted_subarray(vec![1,5,3,2,4]));
        println!("{}", find_unsorted_subarray(vec![1,3,2,4,5]));
        println!("{}", find_unsorted_subarray(vec![2,3,5,4,1]));
        println!("{}", find_unsorted_subarray(vec![2,5,4,1,3]));
    }

    #[test]
    fn solution_583() {
        println!("{}", min_distance("sea".to_string(), "tea".to_string()));
    }
}
