mod topic_832;
mod topic_838;
mod topic_840;

#[cfg(test)]
mod tests {

    use crate::topic_832::flip_and_invert_image;
    use crate::topic_838::push_dominoes;
    use crate::topic_840::num_magic_squares_inside;

    #[test]
    fn solution_832() {
        let a = vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]];
        flip_and_invert_image(a);

        let a = vec![
            vec![1, 1, 0, 0],
            vec![1, 0, 0, 1],
            vec![0, 1, 1, 1],
            vec![1, 0, 1, 0],
        ];
        flip_and_invert_image(a);
    }

    #[test]
    fn solution_838() {
        println!("{}", push_dominoes("RR.L".to_string()));
        println!("{}", push_dominoes(".L.R...LR..L..".to_string()));
    }

    #[test]
    fn solution_840() {
        println!(
            "{}",
            num_magic_squares_inside(vec![vec![4, 3, 8, 4], vec![9, 5, 1, 9], vec![2, 7, 6, 2]])
        );
        println!(
            "{}",
            num_magic_squares_inside(vec![vec![10, 3, 5], vec![1, 6, 11], vec![7, 9, 2]])
        );
    }
}
