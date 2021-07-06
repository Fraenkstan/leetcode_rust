
mod topic_451;

#[cfg(test)]
mod tests {

    use crate::topic_451::frequency_sort;

    #[test]
    fn solution_451() {
        println!("{}", frequency_sort("tree".to_string()));
        println!("{}", frequency_sort("cccaaa".to_string()));
        println!("{}", frequency_sort("Aabb".to_string()));
    }
}
