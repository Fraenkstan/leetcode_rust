mod topic_1104;
mod topic_1109;

#[cfg(test)]
mod tests {

    use crate::topic_1104::path_in_zig_zag_tree;
    use crate::topic_1109::corp_flight_bookings;

    #[test]
    fn solution_1104() {
        println!("{:?}", path_in_zig_zag_tree(14));
        println!("{:?}", path_in_zig_zag_tree(26));
    }

    #[test]
    fn solution_1109() {
        println!(
            "{:?}",
            corp_flight_bookings(vec![vec![1, 2, 10], vec![2, 3, 20], vec![2, 5, 25]], 5)
        );
        println!(
            "{:?}",
            corp_flight_bookings(vec![vec![1, 2, 10], vec![2, 2, 15]], 2)
        );
    }
}
