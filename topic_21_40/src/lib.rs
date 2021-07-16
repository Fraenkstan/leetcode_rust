
mod topic_22;
mod topic_26;
mod topic_27;
mod topic_31;
mod topic_33;
mod topic_34;
mod topic_35;
mod topic_36;
mod topic_39;
mod topic_40;

#[cfg(test)]
mod tests {

    use crate::topic_22::generate_parenthesis;
    use crate::topic_26::{remove_duplicates, remove_duplicates_1};
    use crate::topic_27::remove_element;
    use crate::topic_31::next_permutation;
    use crate::topic_33::search;
    use crate::topic_34::search_range;
    use crate::topic_35::search_insert;
    use crate::topic_36::is_valid_sudoku;
    use crate::topic_39::combination_sum;
    use crate::topic_40::combination_sum2;

    #[test]
    fn solution_22() {
        println!("{:?}", generate_parenthesis(1));
        println!("{:?}", generate_parenthesis(3));
    }

    #[test]
    fn solution_26() {
        println!("{}", remove_duplicates(&mut vec![1, 1, 2]));
        println!("{}", remove_duplicates_1(&mut vec![1, 1, 2]));
        println!("{}", remove_duplicates(&mut vec![0,0,1,1,1,2,2,3,3,4]));
        println!("{}", remove_duplicates_1(&mut vec![0,0,1,1,1,2,2,3,3,4]));
    }

    #[test]
    fn solution_27() {
        println!("{}", remove_element(&mut vec![3,2,2,3], 3));
        println!("{}", remove_element(&mut vec![0,1,2,2,3,0,4,2], 2));
    }

    #[test]
    fn solution_31() {
        let mut test1 = vec![1, 2, 3];
        next_permutation(&mut test1);
        println!("{:?}", test1);

        let mut test2 = vec![3,2,1];
        next_permutation(&mut test2);
        println!("{:?}", test2);

        let mut test3 = vec![1,1,5];
        next_permutation(&mut test3);
        println!("{:?}", test3);

        let mut test4 = vec![1];
        next_permutation(&mut test4);
        println!("{:?}", test4);
    }

    #[test]
    fn solution_33() {
        println!("{}", search(vec![4,5,6,7,0,1,2], 0));
        println!("{}", search(vec![4,5,6,7,0,1,2], 3));
        println!("{}", search(vec![1], 0));
        println!("{}", search(vec![3,5,1], 3));
    }

    #[test]
    fn solution_34() {
        println!("{:?}", search_range(vec![5,7,7,8,8,10], 8));
        println!("{:?}", search_range(vec![5,7,7,8,8,10], 6));
        println!("{:?}", search_range(vec![1,1,1], 1));
        println!("{:?}", search_range(vec![], 0));
        println!("{:?}", search_range(vec![1,4], 4));
        println!("{:?}", search_range(vec![2,2], 3));
    }

    #[test]
    fn solution_35() {
        println!("{}", search_insert(vec![1,3,5,6], 5));
        println!("{}", search_insert(vec![1,3,5,6], 2));
        println!("{}", search_insert(vec![1,3,5,6], 7));
        println!("{}", search_insert(vec![1,3,5,6], 0));
        println!("{}", search_insert(vec![1,3], 0));
        println!("{}", search_insert(vec![1,3], 2));
        println!("{}", search_insert(vec![3,5,7,9,10], 8));
    }

    #[test]
    fn solution_36() {
        let board1 = vec![
            vec!['5','3','.','.','7','.','.','.','.'],
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9']
        ];
        let board2 = vec![
            vec!['8','3','.','.','7','.','.','.','.'],
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9']
        ];
        println!("{}", is_valid_sudoku(board1));
        println!("{}", is_valid_sudoku(board2));
    }

    #[test]
    fn solution_39() {
        println!("{:?}", combination_sum(vec![2,3,6,7], 7));
        println!("{:?}", combination_sum(vec![2,3,5], 8));
    }

    #[test]
    fn solution_40() {
        println!("{:?}", combination_sum2(vec![10,1,2,7,6,1,5], 8));
        println!("{:?}", combination_sum2(vec![2,5,2,1,2], 5));
    }
}
