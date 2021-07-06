
mod topic_1833;

#[cfg(test)]
mod tests {

    use crate::topic_1833::max_ice_cream;

    #[test]
    fn solution_1833() {
        println!("{}", max_ice_cream(vec![1,3,2,4,1], 7));
        println!("{}", max_ice_cream(vec![10,6,8,7,7,8], 5));
        println!("{}", max_ice_cream(vec![1,6,3,1,2,5], 20));
    }
}
