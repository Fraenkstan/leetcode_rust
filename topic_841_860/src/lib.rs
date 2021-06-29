
mod topic_852;

#[cfg(test)]
mod tests {

    use crate::topic_852::peak_index_in_mountain_array;

    #[test]
    fn solution_852() {
        println!("{}", peak_index_in_mountain_array(vec![0,1,0]));
        println!("{}", peak_index_in_mountain_array(vec![0,2,1,0]));
        println!("{}", peak_index_in_mountain_array(vec![24,69,100,99,79,78,67,36,26,19]));
    }
}
