mod topic_17_09;
mod topic_17_10;
mod topic_17_14;

#[cfg(test)]
mod tests {
    use crate::topic_17_09::get_kth_magic_number;
    use crate::topic_17_10::majority_element;
    use crate::topic_17_14::smallest_k;

    #[test]
    fn solution_17_09() {
        println!("{}", get_kth_magic_number(5));
        println!("{}", get_kth_magic_number(7));
    }

    #[test]
    fn solution_17_10() {
        println!("{}", majority_element(vec![1, 2, 5, 9, 5, 9, 5, 5, 5]));
        println!("{}", majority_element(vec![3, 2]));
        println!("{}", majority_element(vec![2, 2, 1, 1, 1, 2,]));
    }

    #[test]
    fn solution_17_14() {
        println!("{:?}", smallest_k(vec![1, 3, 5, 7, 2, 4, 6, 8], 4));
    }
}
