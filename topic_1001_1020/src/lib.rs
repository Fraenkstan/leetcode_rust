
mod topic_1018;

#[cfg(test)]
mod tests {

    use crate::topic_1018::prefixes_div_by5;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn solution_1018() {
        let a = vec![0,1,1,1,1,1];
        println!("{:?}", prefixes_div_by5(a));
    }
}
