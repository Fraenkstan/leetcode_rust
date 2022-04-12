mod topic_802;
mod topic_803;
mod topic_804;
mod topic_806;
mod topic_815;

#[cfg(test)]
mod tests {

    use crate::topic_802::eventual_safe_nodes;
    // use crate::topic_803::hit_bricks;
    use crate::topic_804::unique_morse_representations;
    use crate::topic_806::number_of_lines;
    use crate::topic_815::num_buses_to_destination;

    #[test]
    fn solution_802() {
        println!(
            "{:?}",
            eventual_safe_nodes(vec![
                vec![1, 2],
                vec![2, 3],
                vec![5],
                vec![0],
                vec![5],
                vec![],
                vec![]
            ])
        );
        println!(
            "{:?}",
            eventual_safe_nodes(vec![
                vec![1, 2, 3, 4],
                vec![1, 2],
                vec![3, 4],
                vec![0, 4],
                vec![]
            ])
        );
    }

    #[test]
    fn solution_803() {
        // let grid = vec![vec![1,0,0,0],vec![1,1,1,0]];
        // let hits = vec![vec![1,0]];
        // println!("{:?}", hit_bricks(grid, hits));
    }

    #[test]
    fn solution_804() {
        println!(
            "{}",
            unique_morse_representations(vec![
                "gin".to_string(),
                "zen".to_string(),
                "gig".to_string(),
                "msg".to_string()
            ])
        )
    }

    #[test]
    fn solution_806() {
        println!("{:?}", number_of_lines(vec![10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,
                                            10,10,10,10,10,10,10,10,10,10,10],
                                       "abcdefghijklmnopqrstuvwxyz".to_string()));
        println!("{:?}", number_of_lines(vec![4,10,10,10,10,10,10,10,10,10,10,10,10,10,10,
                                              10,10,10,10,10,10,10,10,10,10,10],
                                         "bbbcccdddaaa".to_string()))
    }

    #[test]
    fn solution_815() {
        println!(
            "{}",
            num_buses_to_destination(vec![vec![1, 2, 7], vec![3, 6, 7]], 1, 6)
        );
        println!(
            "{}",
            num_buses_to_destination(
                vec![
                    vec![7, 12],
                    vec![4, 5, 15],
                    vec![6],
                    vec![15, 19],
                    vec![9, 12, 13]
                ],
                15,
                12
            )
        );
    }
}
