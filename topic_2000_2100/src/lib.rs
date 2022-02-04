
mod topic_2013;
mod topic_2047;

#[cfg(test)]
mod tests {
    use crate::topic_2013::DetectSquares;
    use crate::topic_2047::count_valid_words;

    #[test]
    fn solution_2013() {
        let mut data = DetectSquares::new();
        data.add(vec![3, 10]);
        data.add(vec![11, 2]);
        data.add(vec![3, 2]);
        println!("{}", data.count(vec![11, 10]));
        println!("{}", data.count(vec![14, 8]));
        data.add(vec![11, 2]);
        println!("{}", data.count(vec![11, 10]));
    }

    #[test]
    fn solution_2047() {
        println!("{}", count_valid_words(String::from("cat and  dog")));
        println!("{}", count_valid_words(String::from("!this  1-s b8d!")));
        println!("{}", count_valid_words(String::from("alice and  bob are playing stone-game10")));
        println!("{}", count_valid_words(String::from("he bought 2 pencils, 3 erasers, and 1  pencil-sharpener.")));
    }
}
