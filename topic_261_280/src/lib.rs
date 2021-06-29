
mod topic_264;
mod topic_278;
mod topic_279;

#[cfg(test)]
mod tests {

    use crate::topic_264::nth_ugly_number;
    use crate::topic_279::num_squares;

    #[test]
    fn solution_264() {
        println!("{}", nth_ugly_number(10));
    }

    #[test]
    fn solution_279() {
        println!("{}", num_squares(12));
        println!("{}", num_squares(13));
    }
}
