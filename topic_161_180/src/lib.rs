
mod topic_168;

#[cfg(test)]
mod tests {

    use crate::topic_168::convert_to_title;

    #[test]
    fn solution_168() {
        println!("{}", convert_to_title(1));
        println!("{}", convert_to_title(28));
        println!("{}", convert_to_title(701));
    }
}
