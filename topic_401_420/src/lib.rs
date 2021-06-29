
mod topic_403;

#[cfg(test)]
mod tests {
    use crate::topic_403::{read_binary_watch};

    #[test]
    fn solution_403() {
        for i in 0..10 {
            println!("{:?}", read_binary_watch(i));
        }
    }
}
