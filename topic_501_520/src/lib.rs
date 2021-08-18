
mod topic_503;
mod topic_516;
mod topic_518;

#[cfg(test)]
mod tests {
    use crate::topic_503::next_greater_elements;
    use crate::topic_516::longest_palindrome_subseq;
    use crate::topic_518::change;

    #[test]
    fn solution_503() {
        println!("{:?}", next_greater_elements(vec![]));
    }

    #[test]
    fn solution_516() {
        println!("{}", longest_palindrome_subseq("bbbab".to_string()));
        println!("{}", longest_palindrome_subseq("cbbd".to_string()));
    }

    #[test]
    fn solution_518() {
        println!("{}", change(5, vec![1, 2, 5]));
    }
}
