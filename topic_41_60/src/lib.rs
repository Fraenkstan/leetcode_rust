
mod topic_53;
mod topic_55;

#[cfg(test)]
mod tests {
    use crate::topic_53::max_sub_array;

    #[test]
    fn solution_53() {
        println!("{}", max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]));
        println!("{}", max_sub_array(vec![1]));
        println!("{}", max_sub_array(vec![0]));
        println!("{}", max_sub_array(vec![-1]));
        println!("{}", max_sub_array(vec![-100000]));
    }
}
