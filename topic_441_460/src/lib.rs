mod topic_441;
mod topic_442;
mod topic_443;
mod topic_446;
mod topic_451;
mod topic_457;

#[cfg(test)]
mod tests {

    use crate::topic_441::arrange_coins;
    use crate::topic_442::find_duplicates;
    use crate::topic_443::compress;
    use crate::topic_446::number_of_arithmetic_slices;
    use crate::topic_451::frequency_sort;
    use crate::topic_457::circular_array_loop;

    #[test]
    fn solution_441() {
        println!("{}", arrange_coins(5));
        println!("{}", arrange_coins(8));
        println!("{}", arrange_coins(10));
        println!("{}", arrange_coins(11));
        println!("{}", arrange_coins(2147483647));
    }

    #[test]
    fn solution_442() {
        println!("{:?}", find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]));
    }

    #[test]
    fn solution_443() {
        println!("{}", compress(&mut vec!['a', 'a', 'b', 'b', 'c', 'c', 'c']));
        println!("{}", compress(&mut vec!['a']));
        println!(
            "{}",
            compress(&mut vec![
                'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b'
            ])
        );
    }

    #[test]
    fn solution_446() {
        println!("{}", number_of_arithmetic_slices(vec![2, 4, 6, 8, 10]));
    }

    #[test]
    fn solution_451() {
        println!("{}", frequency_sort("tree".to_string()));
        println!("{}", frequency_sort("cccaaa".to_string()));
        println!("{}", frequency_sort("Aabb".to_string()));
    }

    #[test]
    fn solution_457() {
        println!("{}", circular_array_loop(vec![2, -1, 1, 2, 2]));
        println!("{}", circular_array_loop(vec![-1, -2]));
        println!("{}", circular_array_loop(vec![-2, 1, -1, -2, -2]));
    }
}
