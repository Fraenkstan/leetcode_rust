
mod topic_offer_38;

#[allow(unused)]

#[cfg(test)]
mod tests {

    use crate::topic_offer_38::{permutation, permutation_1};

    #[test]
    fn solution_offer_38() {
        println!("{:?}", permutation("abc".to_string()));
        println!("{:?}", permutation_1("abc".to_string()));
        println!("{:?}", permutation_1("dcz".to_string()));
        println!("{:?}", permutation("dcz".to_string()));
    }
}
