
mod topic_372;

#[cfg(test)]
mod tests {

    use crate::topic_372::{super_pow, super_pow_1};

    #[test]
    pub fn solution_372() {
        let a = 2147483647;
        let b = vec![2, 0, 0];
        println!("{}", super_pow(a, b.clone()));

        println!("{}", super_pow_1(a, b));
    }
}
