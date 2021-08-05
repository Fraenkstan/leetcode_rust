
mod topic_1743;

#[cfg(test)]
mod tests {

    use crate::topic_1743::restore_array;

    #[test]
    fn solution_1743() {
        println!("{:?}", restore_array(vec![vec![2,1], vec![3,4], vec![3,2]]));
        println!("{:?}", restore_array(vec![vec![4,-2], vec![1,4], vec![-3,1]]));
        println!("{:?}", restore_array(vec![vec![100000,-100000]]));
    }
}
