mod topic_867;
mod topic_872;

#[cfg(test)]
mod tests {
    use crate::topic_867::transpose;

    #[test]
    fn solution_867() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        println!("{:?}", transpose(matrix));
    }
}
