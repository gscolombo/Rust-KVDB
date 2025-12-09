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
    #[serde(skip)]
    pub id: Option<u64>,
}

impl Default for BTree {
    fn default() -> Self {
        BTree::new(None)
    }
}

impl BTree {
    pub fn new(root_offset: Option<u64>) -> Self {
        BTree { root: root_offset }
    }

    pub fn insert(&mut self, key: String, value: String, pager: &mut Pager) -> Result<(), Error> {
        let root_offset: u64;

        if let Some(root_id) = self.root {
            let mut root_node = Node::load(root_id, pager)?;

            if root_node.keys.len() == MAX_KEYS {
                let mut new_root = Node::new(false);
                new_root.children.push(root_id); 
                
                new_root.split_child(0, &mut root_node, pager)?;

                let i = if key > new_root.keys[0] { 1 } else { 0 };
                
                let mut child = Node::load(new_root.children[i], pager)?;
                
                // CORREÇÃO: Atualiza o ponteiro do filho modificado na nova raiz
                new_root.children[i] = child.insert_non_full(key, value, pager)?;

                root_offset = new_root.save(pager)?;
                self.root = Some(root_offset);
            } else {
                // CORREÇÃO: A raiz mudou de lugar (append only), atualiza self.root
                root_offset = root_node.insert_non_full(key, value, pager)?;
                self.root = Some(root_offset);
            }
        } else {
            let mut root_node = Node::new(true);
            
            root_offset = root_node.insert_non_full(key, value, pager)?;
            self.root = Some(root_offset);
        }

        pager.update_root_offset(&root_offset.to_be_bytes())
    }

    pub fn search(&self, key: &str, pager: &mut Pager) -> Option<String> {
        if let Some(root_id) = self.root {
            if let Ok(root_node) = Node::load(root_id, pager) {
                return root_node.search(key, pager);
            }
        }
        None
    }

    pub fn delete(&mut self, key: String, pager: &mut Pager) -> Result<(), Error> {
    println!("DEBUG: BTree::delete chamado para chave '{}'", key);
    
    if let Some(root_id) = self.root {
        let mut root_node = Node::load(root_id, pager)?;
        
        // Remove a chave
        root_node.remove_key(key, pager)?;
        
        // Se a raiz ficou vazia após remoção
        if root_node.keys.is_empty() {
            if !root_node.is_leaf {
                // Raiz tem apenas um filho, promover o filho como nova raiz
                let new_root_id = root_node.children[0];
                self.root = Some(new_root_id);
                
                // ATUALIZAR OFFSET DA RAIZ NO ARQUIVO
                pager.update_root_offset(&new_root_id.to_be_bytes())?;
            } else {
                // Árvore ficou vazia
                self.root = None;
                
                // ATUALIZAR OFFSET DA RAIZ PARA 0 (árvore vazia)
                pager.update_root_offset(&0u64.to_be_bytes())?;
            }
        } else {
            // Salva a raiz modificada e atualiza o ponteiro
            let new_root_id = root_node.save(pager)?;
            self.root = Some(new_root_id);
            
            // ATUALIZAR OFFSET DA RAIZ NO ARQUIVO
            pager.update_root_offset(&new_root_id.to_be_bytes())?;
        }
    } else {
        println!("DEBUG: Tentativa de deletar em árvore vazia");
    }
    
    Ok(())
}
}

impl Node {
    pub fn new(is_leaf: bool) -> Self {
        Node {
            keys: Vec::new(),
            values: Vec::new(),
            children: Vec::new(),
            is_leaf,
            id: None,
        }
    }

    pub fn load(offset: u64, pager: &mut Pager) -> Result<Self, Error> {
        let data = pager.read_at(offset, 4096)?;
        let mut node = Node::from_bytes(&data)?;
        node.id = Some(offset);
        Ok(node)
    }

    pub fn save(&self, pager: &mut Pager) -> Result<u64, Error> {
        let mut data = self.to_bytes()?;
        data.resize(4096, 0); 
        
        let offset = pager.get_end_offset()?; 
        pager.write_at(offset, &data)?;
        Ok(offset)
    }

    // CORREÇÃO CRÍTICA: Agora retorna Result<u64, Error> (o novo ID do nó)
    fn insert_non_full(&mut self, key: String, value: String, pager: &mut Pager) -> Result<u64, Error> {
        let mut i = self.keys.len();

        if self.is_leaf {
            while i > 0 && key < self.keys[i - 1] {
                i -= 1;
            }
            self.keys.insert(i, key);
            self.values.insert(i, value);
            // Retorna o novo ID gerado pelo save
            return self.save(pager);
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
                
                // CORREÇÃO: O pai atualiza o ponteiro para o filho que mudou de lugar!
                self.children[i] = correct_child.insert_non_full(key, value, pager)?;
            } else {
                // CORREÇÃO: O pai atualiza o ponteiro para o filho que mudou de lugar!
                self.children[i] = child.insert_non_full(key, value, pager)?;
            }
        }
        // Salva o pai (que agora tem ponteiros atualizados) e retorna seu novo ID
        self.save(pager)
    }

    fn split_child(&mut self, i: usize, child: &mut Node, pager: &mut Pager) -> Result<(), Error> {
        let mut right_node = Node::new(child.is_leaf);

        let split_idx = T; 
        
        let median_key = child.keys[split_idx - 1].clone();
        let median_val = child.values[split_idx - 1].clone();

        right_node.keys = child.keys.drain(split_idx..).collect();
        right_node.values = child.values.drain(split_idx..).collect();

        child.keys.pop();  
        child.values.pop();

        if !child.is_leaf {
            right_node.children = child.children.drain(split_idx..).collect();
        }

        let new_left_id = child.save(pager)?; 
        let right_id = right_node.save(pager)?; 

        self.keys.insert(i, median_key);
        self.values.insert(i, median_val);
        self.children.insert(i + 1, right_id);
        self.children[i] = new_left_id;  // Atualizar ponteiro para filho esquerdo
        
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

    fn remove_key(&mut self, key: String, pager: &mut Pager) -> Result<(), Error> {
            println!("DEBUG: remove_key visitando nó (Leaf={}). Chaves: {:?}", self.is_leaf, self.keys);

        let idx_search = self.keys.binary_search(&key);

        if let Ok(idx) = idx_search {
            println!("DEBUG: Chave '{}' encontrada neste nó no índice {}.", key, idx);
            if self.is_leaf {
                self.keys.remove(idx);
                self.values.remove(idx);
                self.save(pager)?;
            } else {
                self.delete_internal_node(idx, pager)?;
            }
            return Ok(());
        }

        let idx = idx_search.unwrap_err();
        
        if self.is_leaf {
            println!("DEBUG: Nó é folha e chave não encontrada. Fim da linha.");
            return Ok(());
        }

        let child_id = self.children[idx];
        let child_node = Node::load(child_id, pager)?;

        if child_node.keys.len() < T {
            println!("DEBUG: Filho {} tem poucas chaves. Executando fill.", idx);
            self.fill(idx, pager)?;
        }

        let child_idx = match self.keys.binary_search(&key) {
            Ok(_) => unreachable!("Erro de Lógica: Chave apareceu no pai durante a descida"),
            Err(i) => i, 
        };
        println!("DEBUG: Descendo para filho índice {} após verificação de fill.", child_idx);

        let child_id_final = self.children[child_idx];
        let mut child_final = Node::load(child_id_final, pager)?;
        
        child_final.remove_key(key, pager)?;

        // CORREÇÃO DE DELETE: Atualiza ponteiro do filho modificado
        let new_child_id = child_final.save(pager)?;
        self.children[child_idx] = new_child_id;

        Ok(())
    }

    fn delete_internal_node(&mut self, idx: usize, pager: &mut Pager) -> Result<(), Error> {
        let key_to_delete = self.keys[idx].clone();

        let left_child_id = self.children[idx];
        let mut left_child = Node::load(left_child_id, pager)?;

        if left_child.keys.len() >= T {
            let (pred_key, pred_val) = left_child.get_predecessor(pager)?;
            
            self.keys[idx] = pred_key.clone();
            self.values[idx] = pred_val;
            
            left_child.remove_key(pred_key, pager)?;
            
            self.children[idx] = left_child.save(pager)?;
            self.save(pager)?; 

        } else {
            let right_child_id = self.children[idx + 1];
            let mut right_child = Node::load(right_child_id, pager)?;

            if right_child.keys.len() >= T {
                let (succ_key, succ_val) = right_child.get_successor(pager)?;
                
                self.keys[idx] = succ_key.clone();
                self.values[idx] = succ_val;
                
                right_child.remove_key(succ_key, pager)?;
                
                self.children[idx + 1] = right_child.save(pager)?;
                self.save(pager)?;

            } else {
                self.merge(idx, pager)?;
                
                let merged_child_id = self.children[idx];
                let mut merged_child = Node::load(merged_child_id, pager)?;
                
                merged_child.remove_key(key_to_delete, pager)?;
                
                self.children[idx] = merged_child.save(pager)?;
                self.save(pager)?;
            }
        }
        Ok(())
    }

    fn fill(&mut self, idx: usize, pager: &mut Pager) -> Result<(), Error> {
        if idx > 0 {
            let left_sib_id = self.children[idx - 1];
            let left_sib = Node::load(left_sib_id, pager)?;
            if left_sib.keys.len() >= T {
                self.borrow_from_prev(idx, pager)?;
                return Ok(());
            }
        }
        if idx < self.children.len() - 1 {
            let right_sib_id = self.children[idx + 1];
            let right_sib = Node::load(right_sib_id, pager)?;
            if right_sib.keys.len() >= T {
                self.borrow_from_next(idx, pager)?;
                return Ok(());
            }
        }
        if idx < self.children.len() - 1 {
            self.merge(idx, pager)?;
        } else {
            self.merge(idx - 1, pager)?;
        }
        Ok(())
    }

    fn borrow_from_prev(&mut self, idx: usize, pager: &mut Pager) -> Result<(), Error> {
        let child_id = self.children[idx];
        let sibling_id = self.children[idx - 1];

        let mut child = Node::load(child_id, pager)?;
        let mut sibling = Node::load(sibling_id, pager)?;

        child.keys.insert(0, self.keys[idx-1].clone());
        child.values.insert(0, self.values[idx-1].clone());

        self.keys[idx-1]= sibling.keys.pop().unwrap();
        self.values[idx-1] = sibling.values.pop().unwrap();

        if !child.is_leaf {
            let sib_child_ptr = sibling.children.pop().unwrap();
            child.children.insert(0, sib_child_ptr);
        }

        self.children[idx] = child.save(pager)?;
        self.children[idx - 1] = sibling.save(pager)?;
        self.save(pager)?;

        Ok(())
    }

    fn borrow_from_next(&mut self, idx: usize, pager: &mut Pager) -> Result<(), Error> {
        let child_id = self.children[idx];
        let sibling_id = self.children[idx + 1];

        let mut child = Node::load(child_id, pager)?;
        let mut sibling = Node::load(sibling_id, pager)?;

        child.keys.push(self.keys[idx].clone());
        child.values.push(self.values[idx].clone());

        self.keys[idx] = sibling.keys.remove(0);
        self.values[idx] = sibling.values.remove(0);
        
        if !child.is_leaf {
            let sib_child_ptr = sibling.children.remove(0);
            child.children.push(sib_child_ptr);
        }

        self.children[idx] = child.save(pager)?;
        self.children[idx + 1] = sibling.save(pager)?;
        self.save(pager)?;
        Ok(())
    }

    fn merge(&mut self, idx: usize, pager: &mut Pager) -> Result<(), Error> {
        // println!("DEBUG: Realizando Merge no índice {}", idx);
        let left_child_id = self.children[idx];
        let right_child_id = self.children[idx + 1];

        let mut left_child = Node::load(left_child_id, pager)?;
        let right_child = Node::load(right_child_id, pager)?; 

        let median_key = self.keys.remove(idx);
        let median_val = self.values.remove(idx);
        
        left_child.keys.push(median_key);
        left_child.values.push(median_val);

        left_child.keys.extend(right_child.keys);
        left_child.values.extend(right_child.values);

        if !left_child.is_leaf{
            left_child.children.extend(right_child.children);
        }

        self.children.remove(idx + 1);

        self.children[idx] = left_child.save(pager)?;
        self.save(pager)?;
        Ok(())
    }

    fn get_predecessor(&self, pager: &mut Pager) -> Result<(String, String), Error> {
        if self.is_leaf {
            Ok((self.keys.last().unwrap().clone(), self.values.last().unwrap().clone()))

        } else {
            let last_child_id = *self.children.last().unwrap();
            let child = Node::load(last_child_id, pager)?;
            child.get_predecessor(pager)
        }
    }

    fn get_successor(&self, pager: &mut Pager) -> Result<(String, String), Error> {
        if self.is_leaf {
            Ok((self.keys.first().unwrap().clone(), self.values.first().unwrap().clone()))
        } else {
            let first_child_id = self.children[0];
            let child = Node::load(first_child_id, pager)?;
            child.get_successor(pager)
        }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        Ok(bincode::serialize(self).unwrap())
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, Error> {
        Ok(bincode::deserialize(data).unwrap())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::create_database;
    use crate::pager::Pager;
    use std::fs;
    use std::path::Path;

    fn setup_test(db_name: &str) -> (BTree, Pager, String) {
        let filename = format!("./databases/{db_name}.kvdb");
        if Path::new(&filename).exists() {
            fs::remove_file(&filename).unwrap();
        }
        create_database(db_name).expect("Não foi possível criar o banco de dados de teste.");

        let pager = Pager::new(db_name); 
        let btree = BTree::default();
        (btree, pager, filename)
    }

    fn teardown_test(filename: &str) {
        if Path::new(filename).exists() {
            fs::remove_file(filename).unwrap();
        }
    }

    #[test]
    fn test_delete_logic_simple() {
        let (mut tree, mut pager, filename) = setup_test("test_simple_del");

        tree.insert("Key1".to_string(), "Val1".to_string(), &mut pager).unwrap();
        tree.insert("Key2".to_string(), "Val2".to_string(), &mut pager).unwrap();

        tree.delete("Key1".to_string(), &mut pager).unwrap();

        assert_eq!(tree.search("Key1", &mut pager), None);
        assert_eq!(tree.search("Key2", &mut pager), Some("Val2".to_string()));

        teardown_test(&filename);
    }

    #[test]
    fn test_5_debug_mini_stress() {
        let (mut tree, mut pager, filename) = setup_test("test_mini_stress");
        
        println!(">>> INICIANDO INSERÇÃO (0..20) <<<");
        for i in 0..20 {
            let k = format!("{:03}", i);
            tree.insert(k.clone(), k, &mut pager).unwrap();
        }
        
        println!(">>> INICIANDO DELEÇÃO DOS PARES (0, 2, 4... 18) <<<");
        for i in (0..20).step_by(2) {
            let k = format!("{:03}", i);
            println!("--- Deletando {} ---", k);
            tree.delete(k.clone(), &mut pager).unwrap();
            
            if tree.search(&k, &mut pager).is_some() {
                panic!("ERRO CRÍTICO: Acabei de deletar {}, mas ela ainda é encontrada!", k);
            }
        }
        
        println!(">>> VERIFICAÇÃO FINAL <<<");
        for i in 0..20 {
            let k = format!("{:03}", i);
            let resultado = tree.search(&k, &mut pager);

            if i % 2 == 0 {
                if resultado.is_some() {
                    panic!("FALHA FINAL: Chave {} (par) deveria estar deletada, mas foi encontrada.", k);
                }
            } else {
                if resultado.is_none() {
                    panic!("FALHA FINAL: Chave {} (ímpar) deveria existir, mas sumiu.", k);
                }
            }
        }
        
        teardown_test(&filename);
    }

    #[test]
    fn test_delete_integration() {
        let (mut tree, mut pager, filename) = setup_test("test_delete_integration");
        
        // Inserir algumas chaves
        for i in 0..10 {
            let key = format!("key{}", i);
            let value = format!("value{}", i);
            tree.insert(key.clone(), value.clone(), &mut pager).unwrap();
            assert_eq!(tree.search(&key, &mut pager), Some(value));
        }
        
        // Deletar algumas chaves
        tree.delete("key3".to_string(), &mut pager).unwrap();
        assert_eq!(tree.search("key3", &mut pager), None);
        
        tree.delete("key7".to_string(), &mut pager).unwrap();
        assert_eq!(tree.search("key7", &mut pager), None);
        
        // Verificar que as outras ainda existem
        assert_eq!(tree.search("key0", &mut pager), Some("value0".to_string()));
        assert_eq!(tree.search("key9", &mut pager), Some("value9".to_string()));
        
        // Deletar todas as chaves
        for i in 0..10 {
            if i != 3 && i != 7 {  // Já deletamos 3 e 7
                let key = format!("key{}", i);
                tree.delete(key.clone(), &mut pager).unwrap();
                assert_eq!(tree.search(&key, &mut pager), None);
            }
        }
        
        // Verificar que a árvore está vazia
        assert!(tree.root.is_none());
        
        teardown_test(&filename);
    }
}