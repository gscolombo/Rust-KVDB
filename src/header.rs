// src/header.rs
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseHeader {
    pub magic_number: [u8; 4],     // "RUST"
    pub version: u32,              // Versão do formato
    pub root_offset: Option<u64>,  // Onde a raiz da B-Tree está
    pub page_size: u32,            // Tamanho das páginas (4096)
    pub free_list_head: Option<u64>, // Lista de páginas livres
}

impl DatabaseHeader {
    pub fn new() -> Self {
        DatabaseHeader {
            magic_number: [0x52, 0x55, 0x53, 0x54], // "RUST"
            version: 1,
            root_offset: None,
            page_size: 4096,
            free_list_head: None,
        }
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
    
    pub fn from_bytes(data: &[u8]) -> Self {
        bincode::deserialize(data).unwrap()
    }
}