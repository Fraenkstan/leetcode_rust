
pub struct WordsFrequency {
    trie: Trie
}

impl WordsFrequency {
    pub fn new(book: Vec<String>) -> Self {
        let mut trie = Trie::new();
        book.iter().for_each(|word| trie.insert(word));
        Self { trie }
    }

    pub fn get(&self, word: String) -> i32 {
        self.trie.search(word)
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
struct Trie {
    count: i32,
    children: [Option<Box<Trie>>; 26],
}

impl Trie {
    #[inline]
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, word: &String) {
        let mut cur = self;
        for (i, ch) in word.chars().map(|ch| (ch as u8 - 'a' as u8) as usize).enumerate() {
            cur = cur.children[ch].get_or_insert_with(|| Box::new(Trie::new()));
            if i == word.len() - 1 {
                cur.count += 1;
            }
        }
    }

    fn search(&self, word: String) -> i32 {
        let mut cur = self;
        for i in word.chars().map(|ch| (ch as u8 - 'a' as u8) as usize) {
            match cur.children[i].as_ref() {
                Some(node) => cur = node,
                None => return 0,
            }
        }
        cur.count
    }
}