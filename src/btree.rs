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

    pub fn delete(&mut self, key: String, pager: &mut Pager) -> Result<(), Error> {

        if let Some(root_id) = self.root {
            let mut root_node = Node::load(root_id, pager)?;

            root_node.remove_key(key, pager)?;

            if root_node.keys.is_empty() {
                if !root_node.is_leaf {
                    let new_root_id = root_node.children[0];
                    self.root = Some(new_root_id);
                } else {
                    self.root = None;
                }
            } else {
                self.root = Some(root_node.save(pager)?);
            }
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

    fn remove_key(&mut self, key: String, pager: &mut Pager) -> Result<(), Error> {

        let idx_search = self.keys.binary_search(&key);

        if let Ok(idx) = idx_search {
            // Remoção no nó folha - Mais básico, so deletar direto.
            if self.is_leaf {
                self.keys.remove(idx);
                self.values.remove(idx);
                self.save(pager)?;
            } else {
                // Nó interno
                self.delete_internal_node(idx, pager)?;
            }
            return Ok(());
        }

        let idx = idx_search.unwrap_err();

        if self.is_leaf {
            return Ok(());
        }

        let child_id = self.children[idx];
        let child_node = Node::load(child_id, pager)?;

        // Se o numero de chaves for menor que T ,
        // é necessario fazer merge (Qnd a K_min = K),
        // Ou pegar emprestado da esquerda ou direita do nó.
        if child_node.keys.len() < T {
            self.fill(idx, pager)?;
        }

        // Redefinir o indice caso tenha se aplicado o merge
        let child_idx = match self.keys.binary_search(&key) {
            Ok(_) => {
                unreachable!("Erro de Lógica: Chave apareceu no pai durante a descida");
            }
            Err(i) => i, 
        };

        let child_id_final = self.children[child_idx];
        let mut child_final = Node::load(child_id_final, pager)?;
        
        child_final.remove_key(key, pager)?;

        self.children[child_idx] = child_final.save(pager)?;
        self.save(pager)?;

        Ok(())
    }

    fn delete_internal_node(&mut self, idx: usize, pager: &mut Pager) -> Result<(), Error> {
        let key_to_delete = self.keys[idx].clone();

        let left_child_id = self.children[idx];
        let mut left_child = Node::load(left_child_id, pager)?;

        // Se o predecessor tem chaves suficientes, posso pegar a maior chave
        // e substituir a chave.
        if left_child.keys.len() >= T {
            let (pred_key, pred_val) = left_child.get_predecessor(pager)?;
            self.keys[idx] = pred_key.clone();
            self.values[idx] = pred_val;
            self.save(pager)?; 
            left_child.remove_key(pred_key, pager)?;

        } else {
            let right_child_id = self.children[idx + 1];
            let mut right_child = Node::load(right_child_id, pager)?;

            // Se o sucessor tem chaves suficientes, posso pegar a menor chave
            // e substituir a chave.
            if right_child.keys.len() >= T {
                let (succ_key, succ_val) = right_child.get_successor(pager)?;
                self.keys[idx] = succ_key.clone();
                self.values[idx] = succ_val;
                self.save(pager)?;
                right_child.remove_key(succ_key, pager)?;

            } else {
                // Se não tiver como pegar emprestado, é necessário fazer um merge
                // com os nós filhos e a chave do meio sobe para o pai.
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

        // Menor Chave do pai desce para o filho
        child.keys.insert(0, self.keys[idx-1].clone());
        child.values.insert(0, self.values[idx-1].clone());

        // Maior chave do filho sobe para o pai
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

        let left_child_id = self.children[idx];
        let right_child_id = self.children[idx + 1];

        let mut left_child = Node::load(left_child_id, pager)?;
        let right_child = Node::load(right_child_id, pager)?; 

        // Remove a chave mediana do pai
        let median_key = self.keys.remove(idx);
        let median_val = self.values.remove(idx);
        
        // Insere a chave mediana do pai no filho da esquerda
        left_child.keys.push(median_key);
        left_child.values.push(median_val);

        // Merge filho da direta -> esquerda
        left_child.keys.extend(right_child.keys);
        left_child.values.extend(right_child.values);

        // Transferir os nós netos do nó removida para a esquerda 
        if !left_child.is_leaf{
            left_child.children.extend(right_child.children);
        }

        // Remove o ponteiro do filho à direita.
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
    use crate::pager::Pager;
    use std::fs;
    use std::path::Path;

    // Função auxiliar para preparar o ambiente (cria arquivo limpo)
    fn setup_test(db_name: &str) -> (BTree, Pager, String) {
        let filename = format!("{}.db", db_name);
        if Path::new(&filename).exists() {
            fs::remove_file(&filename).unwrap();
        }
        // ATENÇÃO: Pager::new retorna o objeto direto, não um Result. Sem .unwrap()!
        let pager = Pager::new(&filename); 
        let btree = BTree::new();
        (btree, pager, filename)
    }

    // Função auxiliar para limpar a bagunça depois
    fn teardown_test(filename: &str) {
        if Path::new(filename).exists() {
            fs::remove_file(filename).unwrap();
        }
    }

    #[test]
    fn test_simulate_file_storage() {
        // CENÁRIO: Simular o uso da main.rs (Chave + Caminho/Conteúdo)
        let (mut tree, mut pager, filename) = setup_test("test_files");

        // Simula ler arquivos e inserir no banco
        let arquivo_1 = "foto_ferias.png";
        let conteudo_1 = "dados_binarios_da_imagem_......"; // Simulando bytes
        
        let arquivo_2 = "relatorio.pdf";
        let conteudo_2 = "conteudo_do_pdf_importante...";

        // Inserção
        tree.insert(arquivo_1.to_string(), conteudo_1.to_string(), &mut pager).unwrap();
        tree.insert(arquivo_2.to_string(), conteudo_2.to_string(), &mut pager).unwrap();

        // Busca
        assert_eq!(tree.search("foto_ferias.png", &mut pager), Some(conteudo_1.to_string()));
        assert_eq!(tree.search("relatorio.pdf", &mut pager), Some(conteudo_2.to_string()));
        
        // Busca de algo inexistente
        assert_eq!(tree.search("nao_existe.txt", &mut pager), None);

        teardown_test(&filename);
    }

    #[test]
    fn test_delete_logic_simple() {
        // CENÁRIO: Inserir e Deletar básico para garantir que não quebra
        let (mut tree, mut pager, filename) = setup_test("test_simple_del");

        tree.insert("Key1".to_string(), "Val1".to_string(), &mut pager).unwrap();
        tree.insert("Key2".to_string(), "Val2".to_string(), &mut pager).unwrap();

        // Deleta Key1
        tree.delete("Key1".to_string(), &mut pager).unwrap();

        // Verifica
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
            
            // VERIFICAÇÃO IMEDIATA:
            // Vamos garantir que ela sumiu MESMO antes de ir para a próxima
            if tree.search(&k, &mut pager).is_some() {
                panic!("ERRO CRÍTICO: Acabei de deletar {}, mas ela ainda é encontrada!", k);
            }
        }
        
        println!(">>> VERIFICAÇÃO FINAL <<<");
        for i in 0..20 {
            let k = format!("{:03}", i);
            let resultado = tree.search(&k, &mut pager);

            if i % 2 == 0 {
                // Se é par, TEM que ser None
                if resultado.is_some() {
                    panic!("FALHA FINAL: Chave {} (par) deveria estar deletada, mas foi encontrada.", k);
                }
            } else {
                // Se é ímpar, TEM que existir
                if resultado.is_none() {
                    panic!("FALHA FINAL: Chave {} (ímpar) deveria existir, mas sumiu.", k);
                }
            }
        }
        
        teardown_test(&filename);
    }
}