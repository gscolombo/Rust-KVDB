use serde::{Serialize, Deserialize};
use std::io::Error;
use crate::pager::Pager;

// Constantes da B-Tree
const T: usize = 3; 
const MAX_KEYS: usize = 2 * T - 1; 

#[derive(Serialize, Deserialize, Debug)]
pub struct BTree {
    pub root: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub keys: Vec<String>,
    pub values: Vec<String>,
    pub children: Vec<u64>, 
    pub is_leaf: bool,
}

impl BTree {
    pub fn new() -> Self {
        BTree { root: None }
    }

    pub fn insert(&mut self, key: String, value: String, pager: &mut Pager) -> Result<(), Error> {
        if let Some(root_id) = self.root {
            let mut root_node = Node::load(root_id, pager)?;

            if root_node.keys.len() == MAX_KEYS {
                let mut new_root = Node::new(false);
                new_root.children.push(root_id); 
                
                new_root.split_child(0, &mut root_node, pager)?;

                let i = if key > new_root.keys[0] { 1 } else { 0 };
                
                let mut child = Node::load(new_root.children[i], pager)?;
                child.insert_non_full(key, value, pager)?;

                self.root = Some(new_root.save(pager)?);
            } else {
                root_node.insert_non_full(key, value, pager)?;
                self.root = Some(root_node.save(pager)?);
            }
        } else {
            let mut root_node = Node::new(true);
            root_node.insert_non_full(key, value, pager)?;
            self.root = Some(root_node.save(pager)?);
        }
        Ok(())
    }

    pub fn search(&self, key: &str, pager: &mut Pager) -> Option<String> {
        if let Some(root_id) = self.root {
            if let Ok(root_node) = Node::load(root_id, pager) {
                return root_node.search(key, pager);
            }
        }
        None
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

    pub fn load(offset: u64, pager: &mut Pager) -> Result<Self, Error> {
        let data = pager.read_at(offset, 4096)?;
        Node::from_bytes(&data)
    }

    pub fn save(&self, pager: &mut Pager) -> Result<u64, Error> {
        let mut data = self.to_bytes()?;
        // AQUI ESTÁ A CORREÇÃO:
        // Garante que o bloco tenha sempre 4096 bytes preenchendo com zeros
        data.resize(4096, 0); 
        
        let offset = pager.get_end_offset()?; 
        pager.write_at(offset, &data)?;
        Ok(offset)
    }

    fn insert_non_full(&mut self, key: String, value: String, pager: &mut Pager) -> Result<(), Error> {
        let mut i = self.keys.len();

        if self.is_leaf {
            while i > 0 && key < self.keys[i - 1] {
                i -= 1;
            }
            self.keys.insert(i, key);
            self.values.insert(i, value);
            self.save(pager)?; 
        } else {
            while i > 0 && key < self.keys[i - 1] {
                i -= 1;
            }

            let child_id = self.children[i];
            let mut child = Node::load(child_id, pager)?;

            if child.keys.len() == MAX_KEYS {
                self.split_child(i, &mut child, pager)?;

                if key > self.keys[i] {
                    i += 1;
                }
                let mut correct_child = Node::load(self.children[i], pager)?;
                correct_child.insert_non_full(key, value, pager)?;
            } else {
                child.insert_non_full(key, value, pager)?;
            }
        }
        Ok(())
    }

    fn split_child(&mut self, i: usize, child: &mut Node, pager: &mut Pager) -> Result<(), Error> {
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

        let _child_id = child.save(pager)?; 
        let right_id = right_node.save(pager)?; 

        self.keys.insert(i, median_key);
        self.values.insert(i, median_val);

        self.children.insert(i + 1, right_id);
        
        self.save(pager)?;

        Ok(())
    }

    pub fn search(&self, key: &str, pager: &mut Pager) -> Option<String> {
        let mut i = 0;
        while i < self.keys.len() && key > &self.keys[i] {
            i += 1;
        }
        if i < self.keys.len() && key == &self.keys[i] {
            return Some(self.values[i].clone());
        }
        if self.is_leaf {
            return None;
        }
        if let Ok(child_node) = Node::load(self.children[i], pager) {
            return child_node.search(key, pager);
        }
        None
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        Ok(bincode::serialize(self).unwrap())
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, Error> {
        Ok(bincode::deserialize(data).unwrap())
    }
}
