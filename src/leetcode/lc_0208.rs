use std::collections::HashMap;

struct Trie {
    val: Option<String>,
    nodes: HashMap<char, Trie>
}

impl Trie {

    fn new() -> Self {
        Trie { val: None, nodes: HashMap::new() }
    }
    
    fn insert(&mut self, word: String) {
        // if self.nodes.get(k).is_none()
        todo!()
    }
    
    fn search(&self, word: String) -> bool {
        todo!()
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        todo!()
    }
}