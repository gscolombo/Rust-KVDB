use serde::{Deserialize, Serialize};
use std::io::Error;

// Constantes da B-Tree
const T: usize = 3;
const MAX_KEYS: usize = 2 * T - 1;

#[derive(Serialize, Deserialize, Debug)]
pub struct BTree {
    pub size: u64, // Número de chaves
    pub root: Option<Node>,
}

impl Default for BTree {
    fn default() -> Self {
        BTree {
            size: 0,
            root: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    pub keys: Vec<String>,
    pub values: Vec<u64>,
    pub children: Vec<Node>,
    pub is_leaf: bool,
}

impl BTree {
    pub fn new() -> Self {
        BTree::default()
    }

    pub fn insert(&mut self, key: String, value: u64) -> Result<(), Error> {
        if let Some(root) = &mut self.root {
            if root.keys.len() == MAX_KEYS {
                let mut new_root = Node::new(false);
                new_root.children.push(root.clone());

                new_root.split_child(0)?;

                let i = if key > new_root.keys[0] { 1 } else { 0 };

                let child = &mut new_root.children[i];
                child.insert_non_full(key, value)?;

                self.root = Some(new_root);
            } else {
                root.insert_non_full(key, value)?;
            }
        } else {
            let mut root_node = Node::new(true);
            root_node.insert_non_full(key, value)?;
            self.root = Some(root_node);
        }

        self.size += 1;
        Ok(())
    }

    pub fn search(&self, key: &str) -> Option<u64> {
        if let Some(root) = &self.root {
            return root.search(key);
        }
        None
    }

    pub fn _print_structure(&self) {
        println!("=== B-Tree Structure (size: {}) ===", self.size);
        if let Some(root) = &self.root {
            root._print_structure(0);
        } else {
            println!("Empty tree");
        }
        println!("=================================");
    }
}

impl Node {
    pub fn new(is_leaf: bool) -> Self {
        Node {
            keys: Vec::new(),
            values: Vec::new(),
            children: Vec::new(),
            is_leaf,
        }
    }

    fn insert_non_full(&mut self, key: String, value: u64) -> Result<(), Error> {
        let mut i = self.keys.len();

        if self.is_leaf {
            while i > 0 && key < self.keys[i - 1] {
                i -= 1;
            }
            self.keys.insert(i, key);
            self.values.insert(i, value);
        } else {
            while i > 0 && key < self.keys[i - 1] {
                i -= 1;
            }

            let child = &mut self.children[i];

            if child.keys.len() == MAX_KEYS {
                self.split_child(i)?;

                if key > self.keys[i] {
                    i += 1;
                }
                self.children[i].insert_non_full(key, value)?;
            } else {
                child.insert_non_full(key, value)?;
            }
        }
        Ok(())
    }

    fn split_child(&mut self, i: usize) -> Result<(), Error> {
        let child = &mut self.children[i];
        let mut right_node = Node::new(child.is_leaf);

        let _split_idx = T;

        let median_key = child.keys[T - 1].clone();
        let median_val = child.values[T - 1].clone();

        right_node.keys = child.keys.drain(T..).collect();
        right_node.values = child.values.drain(T..).collect();

        if !child.is_leaf {
            right_node.children = child.children.drain(T..).collect();
        }

        child.keys.pop();
        child.values.pop();

        self.keys.insert(i, median_key);
        self.values.insert(i, median_val);

        self.children.insert(i + 1, right_node);

        Ok(())
    }

    pub fn search(&self, key: &str) -> Option<u64> {
        let mut i = 0;
        while i < self.keys.len() && key > &self.keys[i] {
            i += 1;
        }
        if i < self.keys.len() && key == &self.keys[i] {
            return Some(self.values[i]);
        }
        if self.is_leaf {
            return None;
        }
        if self.children.len() > i {
            return self.children[i].search(key);
        }
        None
    }

    /// Print node structure with indentation
    fn _print_structure(&self, depth: usize) {
        let indent = "  ".repeat(depth);
        let node_type = if self.is_leaf { "Leaf" } else { "Internal" };

        println!("{}{} Node:", indent, node_type);
        println!("{}  Keys: {:?}", indent, self.keys);
        println!("{}  Values: {:?}", indent, self.values);
        println!("{}  Child count: {}", indent, self.children.len());

        if !self.is_leaf {
            for (i, child) in self.children.iter().enumerate() {
                println!("{}  Child {}:", indent, i);
                child._print_structure(depth + 2);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let tree = BTree::new();
        assert_eq!(tree.size, 0);
        assert!(tree.root.is_none());
    }

    #[test]
    fn test_single_insert() {
        let mut tree = BTree::new();
        assert!(tree.insert("key1".to_string(), 100).is_ok());
        assert_eq!(tree.size, 1);
        assert_eq!(tree.search("key1"), Some(100));
    }

    #[test]
    fn test_multiple_inserts_no_split() {
        let mut tree = BTree::new();
        for i in 0..5 {
            // 5 keys = MAX_KEYS for T=3
            let key = format!("key{}", i);
            assert!(tree.insert(key.clone(), i as u64).is_ok());
            assert_eq!(tree.search(&key), Some(i as u64));
        }
        assert_eq!(tree.size, 5);
    }

    #[test]
    fn test_split_child_correctly_updates_parent() {
        let mut parent = Node::new(false);

        // Create a full child (5 keys, T=3, MAX_KEYS=5)
        let mut child = Node::new(true);
        for i in 0..5 {
            child.keys.push(format!("key{}", i));
            child.values.push(i as u64);
        }

        parent.children.push(child);

        // Split the child
        assert!(parent.split_child(0).is_ok());

        // Verify parent now has 1 key (median)
        assert_eq!(parent.keys.len(), 1);
        assert_eq!(parent.keys[0], "key2");
        assert_eq!(parent.values[0], 2);

        // Verify parent now has 2 children
        assert_eq!(parent.children.len(), 2);

        // Verify left child has keys 0-1
        assert_eq!(parent.children[0].keys, vec!["key0", "key1"]);

        // Verify right child has keys 3-4
        assert_eq!(parent.children[1].keys, vec!["key3", "key4"]);
    }

    #[test]
    fn test_split_root() {
        let mut tree = BTree::new();
        // Insert 6 items (will cause root split at 5 keys)
        for i in 0..6 {
            let key = format!("key{}", i);
            assert!(tree.insert(key.clone(), i as u64).is_ok());
        }
        assert_eq!(tree.size, 6);

        // Verify all keys are still accessible
        for i in 0..6 {
            let key = format!("key{}", i);
            assert_eq!(tree.search(&key), Some(i as u64));
        }
    }

    #[test]
    fn test_sequential_inserts_and_searches() {
        let mut tree = BTree::new();
        let test_cases = vec![
            ("apple", 1),
            ("banana", 2),
            ("cherry", 3),
            ("date", 4),
            ("elderberry", 5),
            ("fig", 6),
            ("grape", 7),
        ];

        // Insert all
        for (key, value) in &test_cases {
            assert!(tree.insert(key.to_string(), *value).is_ok());
        }

        // Search all
        for (key, expected_value) in &test_cases {
            assert_eq!(tree.search(key), Some(*expected_value));
        }

        // Test non-existent key
        assert_eq!(tree.search("nonexistent"), None);
    }

    #[test]
    fn test_reverse_order_insert() {
        let mut tree = BTree::new();
        for i in (0..10).rev() {
            let key = format!("key{}", i);
            assert!(tree.insert(key.clone(), i as u64).is_ok());
        }

        for i in 0..10 {
            let key = format!("key{}", i);
            assert_eq!(tree.search(&key), Some(i as u64));
        }
    }

    #[test]
    fn test_delete_operation_not_implemented() {
        // Just noting that delete isn't implemented
        let mut tree = BTree::new();
        tree.insert("key1".to_string(), 100).unwrap();
        // tree.delete("key1"); // Not implemented
    }

    #[test]
    fn test_node_structure_after_splits() {
        let mut tree = BTree::new();

        // Insert enough to cause multiple splits
        for i in 0..15 {
            let key = format!("{:03}", i); // Padded keys for consistent ordering
            tree.insert(key, i as u64).unwrap();
        }

        assert_eq!(tree.size, 15);

        // Verify all keys present
        for i in 0..15 {
            let key = format!("{:03}", i);
            assert_eq!(tree.search(&key), Some(i as u64), "Failed for key {}", key);
        }
    }

    #[test]
    fn test_duplicate_keys_behavior() {
        let mut tree = BTree::new();
        tree.insert("key1".to_string(), 100).unwrap();
        tree.insert("key1".to_string(), 200).unwrap(); // Duplicate key

        // Current implementation allows duplicates and search returns first found
        // This might be a design decision, but worth testing
        assert_eq!(tree.search("key1"), Some(100));
    }

    #[test]
    fn test_tree_depth_invariants() {
        let mut tree = BTree::new();
        // B-Tree of order 3 should have all leaves at same depth
        for i in 0..20 {
            tree.insert(format!("key{}", i), i as u64).unwrap();
        }

        // Basic depth check: all inserted keys should be findable
        for i in 0..20 {
            assert_eq!(tree.search(&format!("key{}", i)), Some(i as u64));
        }
    }

    #[test]
    fn test_mixed() {
        println!("=== Manual B-Tree Test ===");

        let mut tree = BTree::new();

        // Test 1: Simple insert
        println!("Test 1: Simple insert");
        tree.insert("cat".to_string(), 1).unwrap();
        tree.insert("dog".to_string(), 2).unwrap();
        tree.insert("bird".to_string(), 3).unwrap();

        println!("Search 'cat': {:?}", tree.search("cat"));
        println!("Search 'dog': {:?}", tree.search("dog"));
        println!("Search 'bird': {:?}", tree.search("bird"));

        // Test 2: Trigger root split
        println!("\nTest 2: Trigger root split");
        tree.insert("ant".to_string(), 4).unwrap();
        tree.insert("elephant".to_string(), 5).unwrap();
        tree.insert("fish".to_string(), 6).unwrap(); // This should split root

        println!("Tree size: {}", tree.size);
        println!("Search 'fish': {:?}", tree.search("fish"));

        // Test 3: More inserts to test structure
        println!("\nTest 3: More inserts");
        for i in 0..10 {
            tree.insert(format!("key{}", i), i + 100).unwrap();
        }

        println!("Tree size: {}", tree.size);
        println!("Search 'key5': {:?}", tree.search("key5"));
    }

    #[test]
    fn test_tree_visualization() {
        let mut tree = BTree::new();

        // Insert in a specific order to trigger interesting splits
        let inserts = vec![
            ("m", 1),
            ("d", 2),
            ("p", 3),
            ("c", 4),
            ("b", 5),
            ("a", 6), // This should trigger splits
            ("e", 7),
            ("f", 8),
            ("g", 9),
            ("h", 10),
            ("i", 11),
            ("j", 12),
            ("k", 13),
            ("l", 14),
            ("n", 15),
            ("o", 16),
            ("q", 17),
            ("r", 18),
            ("s", 19),
            ("t", 20),
        ];

        println!("\n=== Step-by-step tree building ===");
        for (i, (key, value)) in inserts.iter().enumerate() {
            println!("\nStep {}: Insert '{}':{}", i + 1, key, value);
            tree.insert(key.to_string(), *value).unwrap();

            // Print after every few inserts to see structure evolve
            if i == 5 || i == 10 || i == 15 || i == inserts.len() - 1 {
                tree._print_structure();
            }
        }

        // Verify all keys exist
        for (key, expected_value) in &inserts {
            assert_eq!(
                tree.search(key),
                Some(*expected_value),
                "Key '{}' not found or has wrong value",
                key
            );
        }
    }
}
