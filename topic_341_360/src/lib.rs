mod topic_341;
mod topic_342;
mod topic_343;
mod topic_344;
mod topic_345;
mod topic_347;
mod topic_349;
mod topic_350;
mod topic_354;
mod topic_357;

#[cfg(test)]
mod tests {

    use crate::topic_341::{NestedInteger, NestedIterator};
    use crate::topic_342::is_power_of_four;
    use crate::topic_343::integer_break;
    use crate::topic_344::reverse_string;
    use crate::topic_345::reverse_vowels;
    use crate::topic_347::top_k_frequent;
    use crate::topic_349::intersection;
    use crate::topic_350::intersect;
    use crate::topic_354::max_envelopes;
    use crate::topic_357::count_numbers_with_unique_digits;

    #[test]
    fn solution_341() {
        let test1 = vec![
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
            NestedInteger::Int(2),
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
        ];
        let mut iter = NestedIterator::new(test1);
        while iter.has_next() {
            println!("{}", iter.next());
        }
        let test2 = vec![
            NestedInteger::Int(1),
            NestedInteger::List(vec![
                NestedInteger::Int(4),
                NestedInteger::List(vec![NestedInteger::Int(6)]),
            ]),
        ];
        let mut iter = NestedIterator::new(test2);
        while iter.has_next() {
            println!("{}", iter.next());
        }
    }

    #[test]
    fn solution_342() {
        println!("{}", is_power_of_four(16));
        println!("{}", is_power_of_four(5));
        println!("{}", is_power_of_four(1));
        println!("{}", is_power_of_four(-2147483648));
    }

    #[test]
    fn solution_343() {
        println!("{}", integer_break(2));
        println!("{}", integer_break(10));
    }

    #[test]
    fn solution_344() {
        let mut test1 = vec!['h', 'e', 'l', 'l', 'o'];
        let mut test2 = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        reverse_string(&mut test1);
        reverse_string(&mut test2);
        println!("{:?}", test1);
        println!("{:?}", test2);
    }

    #[test]
    fn solution_345() {
        println!("{}", reverse_vowels("hello".to_string()));
        println!("{}", reverse_vowels("leetcode".to_string()));
    }

    #[test]
    fn solution_347() {
        println!("{:?}", top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2));
        println!("{:?}", top_k_frequent(vec![1], 1));
    }

    #[test]
    fn solution_349() {
        println!("{:?}", intersection(vec![1, 2, 2, 1], vec![2, 2]));
        println!("{:?}", intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]));
    }

    #[test]
    fn solution_350() {
        println!("{:?}", intersect(vec![1, 2, 2, 1], vec![2, 2]));
        println!("{:?}", intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]));
    }

    #[test]
    fn solution_354() {
        println!(
            "{}",
            max_envelopes(vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]])
        );
    }

    #[test]
    fn solution_357() {
        // println!("{}", count_numbers_with_unique_digits(0));
        // println!("{}", count_numbers_with_unique_digits(1));
        println!("{}", count_numbers_with_unique_digits(2));
    }
}
