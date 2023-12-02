use fxhash::FxBuildHasher;
use std::collections::HashMap;

// Thanks to Tim! https://dev.to/timclicks/two-trie-implementations-in-rust-ones-super-fast-2f3m

type FxHashMap<K, V> = HashMap<K, V, FxBuildHasher>;

#[derive(Default, Debug)]
pub struct TrieNode {
    pub children: FxHashMap<char, TrieNode>,
    pub is_key: bool,
    pub value: char,
}

#[derive(Default, Debug)]
pub struct Trie {
    pub root: TrieNode,
}

impl Trie {
    pub fn insert(&mut self, word: &str, value: char) {
        let mut node = &mut self.root;
        for c in word.chars() {
            node = node.children.entry(c).or_default();
        }
        node.is_key = true;
        node.value = value;
    }
    pub fn contains(&mut self, word: &str) -> (bool, char) {
        let mut node = &self.root;

        for c in word.chars() {
            if node.is_key {
                return (node.is_key, node.value);
            }
            if c.is_numeric() {
                return (true, c);
            }
            match node.children.get(&c) {
                Some(child_node) => node = child_node,
                None => return (false, node.value),
            }
        }

        (node.is_key, node.value)
    }
}
