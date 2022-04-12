#[cfg(test)]
mod tests {

    use data_structure::trie::word_dictionary::WordDictionary;

    #[test]
    fn test() {
        let mut test = WordDictionary::new();
        test.add_word("bad".to_string());
        test.add_word("dad".to_string());
        test.add_word("mad".to_string());
        println!("{}", test.search("pad".to_string()));
        println!("{}", test.search("bad".to_string()));
        println!("{}", test.search(".ad".to_string()));
        println!("{}", test.search("b..".to_string()));
    }
}
