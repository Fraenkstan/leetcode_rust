mod topic_01_01;
mod topic_01_02;
mod topic_01_03;
mod topic_01_04;
mod topic_01_05;
mod topic_01_06;
mod topic_01_07;
mod topic_01_08;
mod topic_01_09;

#[cfg(test)]
mod tests {
    use crate::topic_01_01::is_unique;
    use crate::topic_01_02::check_permutation;
    use crate::topic_01_03::replace_spaces;
    use crate::topic_01_04::can_permute_palindrome;
    use crate::topic_01_05::one_edit_away;
    use crate::topic_01_06::compress_string;
    use crate::topic_01_07::rotate;
    use crate::topic_01_08::set_zeroes;
    use crate::topic_01_09::is_fliped_string;

    #[test]
    fn solution_01_01() {
        println!("{}", is_unique("leetcode".to_string()));
        println!("{}", is_unique("abc".to_string()));
    }

    #[test]
    fn solution_01_02() {
        println!(
            "{}",
            check_permutation("abc".to_string(), "bca".to_string())
        );
        println!(
            "{}",
            check_permutation("abc".to_string(), "bad".to_string())
        );
    }

    #[test]
    fn solution_01_03() {
        println!("{}", replace_spaces("Mr John Smith    ".to_string(), 13));
        println!("{}", replace_spaces("               ".to_string(), 5));
    }

    #[test]
    fn solution_01_04() {
        println!("{}", can_permute_palindrome("tactcoa".to_string()));
    }

    #[test]
    fn solution_01_05() {
        println!("{}", one_edit_away("pale".to_string(), "ple".to_string()));
        println!("{}", one_edit_away("pales".to_string(), "pal".to_string()));
        println!(
            "{}",
            one_edit_away("teacher".to_string(), "treacher".to_string())
        );
    }

    #[test]
    fn solution_01_06() {
        println!("{}", compress_string("aabcccccaaa".to_string()));
        println!("{}", compress_string("abbccd".to_string()));
    }

    #[test]
    fn solution_01_07() {
        let mut test1 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        rotate(&mut test1);
        println!("{:?}", test1);
        let mut test2 = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        rotate(&mut test2);
        println!("{:?}", test2);
    }

    #[test]
    fn solution_01_08() {
        let mut test1 = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        set_zeroes(&mut test1);
        println!("{:?}", test1);
        let mut test2 = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        set_zeroes(&mut test2);
        println!("{:?}", test2);
    }

    #[test]
    fn solution_01_09() {
        println!(
            "{}",
            is_fliped_string("waterbottle".to_string(), "erbottlewat".to_string())
        );
        println!("{}", is_fliped_string("aa".to_string(), "aba".to_string()));
    }
}
