
mod topic_1;
mod topic_6;

#[cfg(test)]
mod tests {
    use crate::topic_6::convert;

    #[test]
    fn solution_6() {
        println!("{}", convert("PAYPALISHIRING".to_string(), 3));
        println!("{}", convert("PAYPALISHIRING".to_string(), 4));
    }
}