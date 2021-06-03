

#[cfg(test)]
mod tests {

    use data_structure::trie::Trie;

    #[test]
    fn test1() {
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        println!("{}", trie.search("apple".to_string()));
        println!("{}", trie.search("app".to_string()));
        println!("{}", trie.starts_with("app".to_string()));
        trie.insert("app".to_string());
        println!("{}", trie.search("app".to_string()));
    }
}