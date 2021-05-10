
mod topic_832;

#[cfg(test)]
mod tests {

    use crate::topic_832::flip_and_invert_image;

    #[test]
    fn solution_832() {
        let a = vec![vec![1,1,0], vec![1,0,1], vec![0,0,0]];
        flip_and_invert_image(a);

        let a = vec![vec![1,1,0,0], vec![1,0,0,1],
                                    vec![0,1,1,1], vec![1,0,1,0]];
        flip_and_invert_image(a);
    }
}
