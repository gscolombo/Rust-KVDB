use crate::pager::Pager;
use crate::btree::BTree;
use crate::records::{create_record, serialize_record};
use std::io::Error;

pub struct KVDB {
    btree: BTree,
    pager: Pager,
}

impl KVDB {
    pub fn open(filename: &str) -> Result<Self, Error> {
        let mut pager = Pager::new(filename);
        let btree = BTree::load_from_pager(&mut pager)?;
        
        Ok(KVDB { btree, pager })
    }
    
    pub fn create_new(filename: &str) -> Result<Self, Error> {
        let pager = Pager::new(filename);
        let btree = BTree::new();
        
        Ok(KVDB { btree, pager })
    }
    
    pub fn put(&mut self, key: &str, file_path: &str) -> Result<(), Error> {
        // Cria o record a partir do arquivo
        let key_str = key.to_string();
        let path_str = file_path.to_string();
        let record = create_record(&key_str, &path_str);
        
        // Serializa o record para bytes
        let data = serialize_record(record);
        
        // Armazena bytes diretamente na B-Tree
        self.btree.insert(key.to_string(), data, &mut self.pager)?;
        
        // Persiste a raiz
        self.btree.save_to_pager(&mut self.pager)?;
        
        Ok(())
    }
    
    pub fn get(&mut self, key: &str) -> Result<Option<Vec<u8>>, Error> {
        Ok(self.btree.search(key, &mut self.pager))
    }
    
    pub fn delete(&mut self, key: &str) -> Result<(), Error> {
        self.btree.delete(key.to_string(), &mut self.pager)?;
        self.btree.save_to_pager(&mut self.pager)?;
        Ok(())
    }
}