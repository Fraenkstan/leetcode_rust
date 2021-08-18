
mod topic_551;
mod topic_552;
mod topic_553;

#[cfg(test)]
mod tests {
    use crate::topic_551::check_record;
    use crate::topic_552::check_record as check_record_2;
    use crate::topic_553::optimal_division;

    #[test]
    fn solution_551() {
        println!("{}", check_record("PPALLP".to_string()));
        println!("{}", check_record("PPALLL".to_string()));
    }

    #[test]
    fn solution_552() {
        println!("{}", check_record_2(2));
        println!("{}", check_record_2(1));
        println!("{}", check_record_2(10101));
    }

    #[test]
    fn solution_553() {
        println!("{}", optimal_division(vec![1000, 100, 10, 2]));
    }
}
