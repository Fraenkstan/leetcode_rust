
mod topic_lcp_7;

#[cfg(test)]
mod tests {

    use crate::topic_lcp_7::num_ways;

    #[test]
    fn solution_lcp_7() {
        println!("{}", num_ways(5, vec![vec![0,2],vec![2,1],vec![3,4],
                                        vec![2,3],vec![1,4],vec![2,0],vec![0,4]], 3));
        println!("{}", num_ways(3, vec![vec![0,2],vec![2,1]], 2));
    }
}
