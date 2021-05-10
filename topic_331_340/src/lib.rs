
mod topic_338;

#[cfg(test)]
mod tests {
    use crate::topic_338::count_bits;

    #[test]
    fn solution_338() {
        println!("{:?}", count_bits(5));
        println!("{:?}", count_bits(9));
    }
}
