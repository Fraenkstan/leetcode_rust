
mod topic_301;
mod topic_303;
mod topic_313;
mod topic_304;

#[cfg(test)]
mod tests {
    use crate::topic_301::remove_invalid_parentheses;
    use crate::topic_313::nth_super_ugly_number;

    #[test]
    fn solution_301() {

    }

    #[test]
    fn solution_313() {
        println!("{}", nth_super_ugly_number(12, vec![2,7,13,19]));
        println!("{}", nth_super_ugly_number(1, vec![2,3,5]));
    }
}
