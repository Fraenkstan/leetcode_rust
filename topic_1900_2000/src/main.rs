fn main() {
    println!("Hello, world!");
}

mod topic_1984;
mod topic_1996;
mod topic_2000;

mod test {
    use crate::topic_1984::minimum_difference;

    #[test]
    fn solution_1984() {
        println!("{}", minimum_difference(vec![90], 1));
        println!("{}", minimum_difference(vec![9, 4, 1, 7], 2));
    }
}
