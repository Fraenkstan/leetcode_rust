
mod topic_08_01;

#[cfg(test)]
mod tests {
    use crate::topic_08_01::ways_to_step;

    #[test]
    fn solution_08_01() {
        println!("{}", ways_to_step(1));
        println!("{}", ways_to_step(3));
        println!("{}", ways_to_step(5));
    }
}
