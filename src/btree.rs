const M: usize = 5; //ordem máxima da árvore B
const MAX_KEYS: usize = M - 1; //número máximo de chaves por nó
const MIN_KEYS: usize = M / 2; //número mínimo de chaves por nó   
const T: usize = M / 2; //número mínimo de filhos por nó

pub struct BTree {
    pub root: Option<Box<Node>>,
}
pub struct Node {
    pub keys: Vec<String>,
    pub values: Vec<String>,
    pub children: Vec<Box<Node>>,    //nós filhos (usa o box para guardar na heap pq tem tamanho indefinido)
    pub is_leaf: bool,  //determina se é folha ou não
}
struct SplitNodeResult {
    pub median_key: String,        // mediana da chave
    pub median_value: String,      // mediana do valor
    pub right_node: Box<Node>,    // novo nó à direita do atual
}

impl BTree {
    pub fn new() -> Self {
        BTree{
            root: Some(Box::new(Node::new(true))), //determina inicialmente o nó como folha e chma new do nó
        }
    }
    pub fn insert(&mut self, key: String, value: String) {

        if let Some(root_node) = self.root.as_mut() {
            if let Some(split_result) = root_node.insert_node(key, value) {
                let mut new_root = Node::new(false);
                new_root.keys.push(split_result.median_key);
                new_root.values.push(split_result.median_value);

                new_root.children.push(self.root.take().unwrap());
                new_root.children.push(split_result.right_node);

                self.root = Some(Box::new(new_root));

            }
        }
        else {
            let mut new_root = Node::new(true);
            new_root.insert_node(key, value);
            return self.root = Some(Box::new(new_root));
        }                                                              
    }
    pub fn delete(&mut self, key: &str) {
        if self.root.is_none() {
            return;
        }
        let root = self.root.as_mut().unwrap();
        root.remove(key);

        if root.keys.is_empty() {
            if root.is_leaf {
                self.root = None;
            } else {
                self.root = Some(root.children.remove(0));
            }
        }
    }
}

//Criar um novo nó
impl Node {
    pub fn new(is_leaf: bool) -> Self {
        Node {
            //cria vetores vazios com os tipos certos, compilador entende tipo pelo contexto
            keys: Vec::new(),
            values: Vec::new(),
            children: Vec::new(),
            is_leaf,  //valor do parametro
        }
    }

    pub fn search_node(&self, key: &str) -> Option<&String> { //talvez exista um valor (retorna Some ou None)
        //encontra a chave no vetor keys
        let mut i = 0;
        while i < self.keys.len() && key > &self.keys[i] {
            i += 1;
        }

        //encontra a chave
        if i < self.keys.len() && key == &self.keys[i] {
            return Some(&self.values[i]);
        }

        //se for folha
        if self.is_leaf {
            return None;                                                               
        }

        //se não for folha, busca no children 
        self.children[i].search_node(key)
    }

    fn insert_node(&mut self, key: String, value : String) -> Option<SplitNodeResult> {

        let i = self.find_key_index(&key);

        if i < self.keys.len() && self.keys[i] == key {
            self.values[i] = value;
            return None;
        }

        if self.is_leaf {
            self.keys.insert(i,key);
            self.values.insert(i,value);

        }
        else {
            let child = &mut self.children[i];
            if let Some(split_result) = child.insert_node(key, value) {
                self.keys.insert(i, split_result.median_key);
                self.values.insert(i, split_result.median_value);
                self.children.insert(i + 1, split_result.right_node);
            }
        }

        if self.keys.len() > MAX_KEYS {
            return Some(self.split_node());
        } 
        return None;
    }

    fn split_node(&mut self) -> SplitNodeResult {

        let mut right_node = Node::new(self.is_leaf);
        right_node.keys = self.keys.split_off(T + 1);
        right_node.values = self.values.split_off(T + 1);

        if !self.is_leaf {
            right_node.children = self.children.split_off(T+1);
        }

        SplitNodeResult {
            median_key: self.keys.pop().unwrap(),
            median_value: self.values.pop().unwrap(),
            right_node: Box::new(right_node),
        }
    }
//
    fn remove(&mut self, key: &str) {
        let index = self.find_key_index(key);

        if index < self.keys.len() && self.keys[index] == key {
            if self.is_leaf {
                self.remove_from_leaf(index);
            } 
            else {
                self.remove_from_internal(index);
            }
        } 
        else {
            if self.is_leaf {
                return; // A chave não está presente na árvore
            }

            if self.children[index].keys.len() <= MIN_KEYS {
                self.fill(index);
            }

            let mut new_index = index;
            if new_index >= self.children.len() {
                new_index = self.children.len() - 1;
            } 
            else if new_index < self.keys.len() && key > &self.keys[new_index] {
                 new_index += 1;
            }
            
            self.children[new_index].remove(key);
        }
    }

    fn remove_from_leaf(&mut self, index: usize) {
        self.keys.remove(index);
        self.values.remove(index);
    }

    fn remove_from_internal(&mut self, index: usize) {
        let key = self.keys[index].clone();

        if self.children[index].keys.len() > MIN_KEYS {
            let (pred_key, pred_value) = self.get_predecessor(index);
            self.keys[index] = pred_key.clone();
            self.values[index] = pred_value;

            self.children[index].remove(&pred_key);

        }
        else if self.children[index + 1].keys.len() > MIN_KEYS {
            let (succ_key, succ_value) = self.get_successor(index);
            self.keys[index] = succ_key.clone();
            self.values[index] = succ_value;
            self.children[index + 1].remove(&succ_key);
        } 
        else {
            self.merge(index);
            self.children[index].remove(&key);
        }
    }
    
    fn find_key_index(&self, key: &str) -> usize {
        let mut index = 0;
        while index < self.keys.len() && key > self.keys[index].as_str(){
            index += 1;
        }
        return index;
    }

    fn get_predecessor(&self, index: usize) -> (String, String) {
        let mut current = &self.children[index];
        while !current.is_leaf {
            current = &current.children[current.keys.len()];
        }
        (current.keys[current.keys.len() - 1].clone(), current.values[current.values.len() - 1].clone())
    }

    fn get_successor(&self, index: usize) -> (String, String) {
        let mut current = &self.children[index + 1];
        while !current.is_leaf {
            current = &current.children[0];
        }
        (current.keys[0].clone(), current.values[0].clone())
    }

    fn merge(&mut self, index: usize) {
        let right_child = self.children.remove(index + 1);
        let child = &mut self.children[index];

        let median_key = self.keys.remove(index);
        let median_value = self.values.remove(index);

        child.keys.push(median_key);
        child.values.push(median_value);

        child.keys.extend(right_child.keys);
        child.values.extend(right_child.values);
        if !child.is_leaf {
            child.children.extend(right_child.children);
        }
    }

    fn borrow_from_prev(&mut self, index: usize) {
        let (left,right) = self.children.split_at_mut(index);
        let sibling = &mut left[index-1];
        let child = &mut right[0];

        let parent_key = self.keys.remove(index - 1);
        let parent_value = self.values.remove(index - 1);

        child.keys.insert(0, parent_key);
        child.values.insert(0, parent_value);

        let sibling_key = sibling.keys.pop().unwrap();
        let sibling_value = sibling.values.pop().unwrap();

        self.keys.insert(index - 1, sibling_key);
        self.values.insert(index - 1, sibling_value);

        if !child.is_leaf {
            let sibling_child = sibling.children.pop().unwrap();
            child.children.insert(0, sibling_child);
        }
    }

    fn borrow_from_next(&mut self, index: usize) {
        let (left, right) = self.children.split_at_mut(index + 1);
        let child = &mut left[index];
        let sibling = &mut right[0];

        let parent_key = self.keys.remove(index);
        let parent_value = self.values.remove(index);
        child.keys.push(parent_key);
        child.values.push(parent_value);

        let sibling_key = sibling.keys.remove(0);
        let sibling_value = sibling.values.remove(0);
        self.keys.insert(index, sibling_key);
        self.values.insert(index, sibling_value);

        if !child.is_leaf {
            let sibling_child = sibling.children.remove(0);
            child.children.push(sibling_child);
        }
    }

    fn fill(&mut self, index: usize) {
        if index != 0 && self.children[index - 1].keys.len() >= MIN_KEYS {
            self.borrow_from_prev(index);
        } 
        else if index != self.keys.len() && self.children[index + 1].keys.len() >= MIN_KEYS {
            self.borrow_from_next(index);
        } 
        else {
            if index != self.keys.len() {
                self.merge(index);
            } 
            else {
                self.merge(index - 1);
            }
        }
    }
}
