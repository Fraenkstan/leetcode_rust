
mod topic_258;

#[cfg(test)]
mod tests {
    use crate::topic_258::add_digits;

    #[test]
    fn solution_258() {
        println!("{}", add_digits(38));
        println!("{}", add_digits(0));
    }
}
