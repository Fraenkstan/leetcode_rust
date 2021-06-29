
mod topic_503;
mod topic_518;

#[cfg(test)]
mod tests {
    use crate::topic_518::change;

    #[test]
    fn solution_518() {
        println!("{}", change(5, vec![1, 2, 5]));
    }
}
