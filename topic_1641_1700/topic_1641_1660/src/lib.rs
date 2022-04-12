mod topic_1646;

#[cfg(test)]
mod tests {

    use crate::topic_1646::get_maximum_generated;

    #[test]
    fn solution_1646() {
        println!("{}", get_maximum_generated(7));
        println!("{}", get_maximum_generated(2));
        println!("{}", get_maximum_generated(3));
    }
}
