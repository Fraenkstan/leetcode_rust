
mod topic_803;
mod topic_815;

#[cfg(test)]
mod tests {

    // use crate::topic_803::hit_bricks;
    use crate::topic_815::num_buses_to_destination;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn solution_803() {
        // let grid = vec![vec![1,0,0,0],vec![1,1,1,0]];
        // let hits = vec![vec![1,0]];
        // println!("{:?}", hit_bricks(grid, hits));
    }

    #[test]
    fn solution_815() {
        println!("{}", num_buses_to_destination(vec![vec![1,2,7], vec![3,6,7]], 1, 6));
        println!("{}", num_buses_to_destination(vec![vec![7,12], vec![4,5,15], vec![6],
                                                     vec![15,19], vec![9,12,13]], 15, 12));
    }
}
