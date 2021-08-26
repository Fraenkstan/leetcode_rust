
mod topic_541;
mod topic_542;

#[cfg(test)]
mod tests {

    use crate::topic_541::reverse_str;
    use crate::topic_542::update_matrix;

    #[test]
    fn solution_541() {
        println!("{}", reverse_str("abcdefg".to_string(), 2));
        println!("{}", reverse_str("abcd".to_string(), 2));
    }

    #[test]
    fn solution_542() {
        println!("{:?}", update_matrix(vec![vec![0,0,0], vec![0,1,0], vec![0,0,0]]));
        println!("{:?}", update_matrix(vec![vec![0,0,0], vec![0,1,0], vec![1,1,1]]));
    }
}
