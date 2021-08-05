
mod topic_1337;

#[cfg(test)]
mod tests {

    use crate::topic_1337::k_weakest_rows;

    #[test]
    fn solution_1337() {
        println!("{:?}", k_weakest_rows(vec![vec![1,1,0,0,0], vec![1,1,1,1,0], vec![1,0,0,0,0],
                                             vec![1,1,0,0,0], vec![1,1,1,1,1]], 3));
        println!("{:?}", k_weakest_rows(vec![vec![1,0,0,0], vec![1,1,1,1], vec![1,0,0,0], vec![1,0,0,0]], 2));
    }
}
