
mod topic_1104;

#[cfg(test)]
mod tests {

    use crate::topic_1104::path_in_zig_zag_tree;

    #[test]
    fn solution_1104() {
        println!("{:?}", path_in_zig_zag_tree(14));
        println!("{:?}", path_in_zig_zag_tree(26));
    }
}
