
mod topic_981;

#[cfg(test)]
mod tests {

    use crate::topic_981::TimeMap;

    #[test]
    fn solution_981() {
        let mut test = TimeMap::new();
        test.set("foo".to_string(), "bar".to_string(), 1);
        println!("{}", test.get("foo".to_string(), 1));
        println!("{}", test.get("foo".to_string(), 3));
        test.set("foo".to_string(), "bar2".to_string(), 4);
        println!("{}", test.get("foo".to_string(), 4));
        println!("{}", test.get("foo".to_string(), 5));
    }
}
