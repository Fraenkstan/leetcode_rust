mod topic_45;
mod topic_53;
mod topic_55;

#[cfg(test)]
mod tests {
    use crate::topic_45::jump;
    use crate::topic_53::max_sub_array;
    use crate::topic_55::can_jump;

    #[test]
    fn solution_45() {
        println!("{}", jump(vec![2, 3, 1, 1, 4]));
        println!("{}", jump(vec![2, 3, 0, 1, 4]));
    }

    #[test]
    fn solution_53() {
        println!("{}", max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]));
        println!("{}", max_sub_array(vec![1]));
        println!("{}", max_sub_array(vec![0]));
        println!("{}", max_sub_array(vec![-1]));
        println!("{}", max_sub_array(vec![-100000]));
    }

    #[test]
    fn solution_55() {
        println!("{}", can_jump(vec![2, 3, 1, 1, 4]));
        println!("{}", can_jump(vec![3, 2, 1, 0, 4]));
    }
}
