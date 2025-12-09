use std::fs::{File};
use std::io::{Read, Write, Seek, SeekFrom, Result};

use crate::db::open_database;

pub struct Pager {
    file: File,
}

impl Pager {
    // Abre ou cria o arquivo do banco de dados
    pub fn new(db_name: &str) -> Self {
        let file = open_database(db_name);

        Pager { file }
    }

    // Escreve bytes em uma posição específica (offset)
    pub fn write_at(&mut self, offset: u64, data: &[u8]) -> Result<()> {
        self.file.seek(SeekFrom::Start(offset))?;
        self.file.write_all(data)?;
        Ok(())
    }

    // Lê bytes de uma posição específica
    pub fn read_at(&mut self, offset: u64, length: usize) -> Result<Vec<u8>> {
        self.file.seek(SeekFrom::Start(offset))?;
        let mut buffer = vec![0; length];
        self.file.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    // Descobre onde escrever o próximo nó (no final do arquivo)
    pub fn get_end_offset(&mut self) -> Result<u64> {
        self.file.seek(SeekFrom::End(0))
    }

    pub fn update_root_offset(&mut self, root_offset: &[u8;8]) -> Result<()> {
        self.write_at(0, root_offset)
    }
}