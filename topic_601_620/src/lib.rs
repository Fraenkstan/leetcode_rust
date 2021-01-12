mod topic_605;

#[cfg(test)]
mod tests {

    use crate::topic_605::can_place_flowers;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn solution_605() {
        println!("Hello, world!");
        let flowerbed = vec![1, 0, 0, 0, 1];
        let result = can_place_flowers(flowerbed, 2);
        println!("{}", result);
    }
}
