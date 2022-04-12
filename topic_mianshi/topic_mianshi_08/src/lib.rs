mod topic_08_01;
mod topic_08_02;
mod topic_08_03;
mod topic_08_04;
mod topic_08_05;
mod topic_08_06;
mod topic_08_07;
mod topic_08_08;
mod topic_08_09;
mod topic_08_10;
mod topic_08_11;
mod topic_08_12;
mod topic_08_13;
mod topic_08_14;

#[cfg(test)]
mod tests {
    use crate::topic_08_01::ways_to_step;
    use crate::topic_08_02::path_with_obstacles;
    use crate::topic_08_03::find_magic_index;
    use crate::topic_08_04::subsets;
    use crate::topic_08_05::multiply;
    use crate::topic_08_06::hanota;
    use crate::topic_08_07::permutation;
    use crate::topic_08_08::permutation as permutation1;
    use crate::topic_08_09::generate_parenthesis;
    use crate::topic_08_10::flood_fill;
    use crate::topic_08_11::ways_to_change;
    use crate::topic_08_12::solve_n_queens;
    use crate::topic_08_13::pile_box;
    use crate::topic_08_14::count_eval;

    #[test]
    fn solution_08_01() {
        println!("{}", ways_to_step(1));
        println!("{}", ways_to_step(3));
        println!("{}", ways_to_step(5));
    }

    #[test]
    fn solution_08_02() {
        println!(
            "{:?}",
            path_with_obstacles(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]])
        );
    }

    #[test]
    fn solution_08_03() {
        println!("{}", find_magic_index(vec![0, 2, 3, 4, 5]));
        println!("{}", find_magic_index(vec![1, 1, 1]));
    }

    #[test]
    fn solution_08_04() {
        println!("{:?}", subsets(vec![1, 2, 3]));
        println!("{:?}", subsets(vec![1, 1, 1]));
    }

    #[test]
    fn solution_08_05() {
        println!("{}", multiply(1, 10));
        println!("{}", multiply(3, 4));
    }

    #[test]
    fn solution_08_06() {
        let mut a = vec![2, 1, 0];
        let mut b = vec![];
        let mut c = vec![];
        hanota(&mut a, &mut b, &mut c);
        println!("{:?}", c);
    }

    #[test]
    fn solution_08_07() {
        println!("{:?}", permutation("qwe".to_string()));
    }

    #[test]
    fn solution_08_08() {
        println!("{:?}", permutation1("qqe".to_string()));
    }

    #[test]
    fn solution_08_09() {
        println!("{:?}", generate_parenthesis(3));
    }

    #[test]
    fn solution_08_10() {
        println!(
            "{:?}",
            flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2)
        );
        println!(
            "{:?}",
            flood_fill(vec![vec![0, 0, 0], vec![0, 0, 0]], 0, 0, 1)
        );
        println!(
            "{:?}",
            flood_fill(vec![vec![0, 0, 0], vec![0, 1, 1]], 1, 1, 1)
        );
    }

    #[test]
    fn solution_08_11() {
        println!("{}", ways_to_change(5));
        println!("{}", ways_to_change(10));
        println!("{}", ways_to_change(24));
        println!("{}", ways_to_change(25));
        println!("{}", ways_to_change(61));
    }

    #[test]
    fn solution_08_12() {
        let test = solve_n_queens(8);
        for (i, strs) in test.iter().enumerate() {
            println!("🤡🤡🤡第{}个matrix : ", i);
            for str in strs.iter() {
                println!("{}", str);
            }
        }
    }

    #[test]
    fn solution_08_13() {
        println!(
            "{}",
            pile_box(vec![vec![1, 1, 1], vec![2, 2, 2], vec![3, 3, 3]])
        );
        println!(
            "{}",
            pile_box(vec![
                vec![1, 1, 1],
                vec![2, 3, 4],
                vec![2, 6, 7],
                vec![3, 4, 5]
            ])
        );
        println!(
            "{}",
            pile_box(vec![vec![16, 20, 17], vec![16, 18, 10], vec![11, 11, 19]])
        );
    }

    #[test]
    fn solution_08_14() {
        println!("{}", count_eval("1^0|0|1".to_string(), 0));
        println!("{}", count_eval("0&0&0&1^1|0".to_string(), 1));
    }
}
