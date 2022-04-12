mod topic_1018;
mod topic_1020;

#[cfg(test)]
mod tests {
    use crate::topic_1018::prefixes_div_by5;
    use crate::topic_1020::num_enclaves;

    #[test]
    fn solution_1018() {
        let a = vec![0, 1, 1, 1, 1, 1];
        println!("{:?}", prefixes_div_by5(a));
    }

    #[test]
    fn solution_1020() {
        let grid = vec![
            vec![0, 0, 0, 0],
            vec![1, 0, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 0],
        ];
        println!("{}", num_enclaves(grid));
        let grid = vec![
            vec![0, 1, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 0],
        ];
        println!("{}", num_enclaves(grid));
    }
}
