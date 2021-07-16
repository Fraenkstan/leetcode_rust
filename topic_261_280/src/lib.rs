
mod topic_264;
mod topic_274;
mod topic_275;
mod topic_278;
mod topic_279;

#[cfg(test)]
mod tests {

    use crate::topic_264::nth_ugly_number;
    use crate::topic_274::h_index;
    use crate::topic_275::h_index as h_index_2;
    use crate::topic_278::first_bad_version;
    use crate::topic_279::num_squares;

    #[test]
    fn solution_264() {
        println!("{}", nth_ugly_number(10));
    }

    #[test]
    fn solution_274() {
        println!("{}", h_index(vec![3,0,6,1,5]));
    }

    #[test]
    fn solution_275() {
        println!("{}", h_index_2(vec![0,1,3,5,6]));
    }

    #[test]
    fn solution_278() {
        println!("{}", first_bad_version(5));
        println!("{}", first_bad_version(1));
    }

    #[test]
    fn solution_279() {
        println!("{}", num_squares(12));
        println!("{}", num_squares(13));
    }
}
