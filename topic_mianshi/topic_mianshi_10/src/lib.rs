
mod topic_10_01;
mod topic_10_02;
mod topic_10_03;
mod topic_10_05;
mod topic_10_09;
mod topic_10_10;
mod topic_10_11;

#[cfg(test)]
mod tests {
    use crate::topic_10_01::merge;
    use crate::topic_10_02::group_anagrams;
    use crate::topic_10_03::search;
    use crate::topic_10_05::find_string;
    use crate::topic_10_09::search_matrix;
    use crate::topic_10_11::wiggle_sort;

    #[test]
    fn solution_10_01() {
        let mut a = vec![1,2,3,0,0,0];
        let mut b = vec![2,5,6];
        merge(&mut a, 3, &mut b, 3);
        println!("{:?}", a);
    }

    #[test]
    fn solution_10_02() {
        println!("{:?}", group_anagrams(vec!["eat".to_string(), "tea".to_string(), "tan".to_string(),
                                                  "ate".to_string(), "nat".to_string(), "bat".to_string()]));
    }

    #[test]
    fn solution_10_03() {
        println!("{}", search(vec![15,16,19,20,25,1,3,4,5,7,10,14], 5));
        println!("{}", search(vec![15,16,19,20,25,1,3,4,5,7,10,14], 11));
        println!("{}", search(vec![1,1,1,1,1,2,1,1,1], 2));
        println!("{}", search(vec![5,5,5,1,2,3,4,5], 5));
    }

    #[test]
    fn solution_10_05() {
        println!("{}", find_string(vec!["at".to_string(), "".to_string(), "".to_string(),
                                        "".to_string(), "ball".to_string(), "".to_string(), "".to_string(),
                                        "car".to_string(), "".to_string(), "".to_string(), "dad".to_string(),
                                        "".to_string(), "".to_string()],
                                   "ta".to_string()));
        println!("{}", find_string(vec!["at".to_string(), "".to_string(), "".to_string(),
                                        "".to_string(), "ball".to_string(), "".to_string(), "".to_string(),
                                        "car".to_string(), "".to_string(), "".to_string(), "dad".to_string(),
                                        "".to_string(), "".to_string()],
                                   "ball".to_string()));
        println!("{}", find_string(vec!["CitZMIXZKoFbxvOlaza".to_string(), "hBlKXdKJfBD".to_string()],
                                   "hBlKXdKJfBD".to_string()))
    }

    #[test]
    fn solution_10_09() {
        let matrix = vec![vec![1,4,7,11,15], vec![2,5,8,12,19], vec![3,6,9,16,22],
                                         vec![10,13,14,17,24], vec![18,21,23,26,30]];
        println!("{}", search_matrix(&matrix, 5));
        println!("{}", search_matrix(&matrix, 10));
    }

    #[test]
    fn solution_10_11() {
        let mut test = vec![5,3,1,2,3];
        wiggle_sort(&mut test);
        println!("{:?}", test);
    }
}
