use std::fs::{File};
use std::io::{Read, Write, Seek, SeekFrom, Result};

use crate::db::open_database;

/// Gerenciador de acesso paginado ao arquivo do banco de dados
/// 
/// Responsável por ler e escrever dados em posições específicas (offsets)
/// do arquivo, abstraindo operações de I/O de baixo nível.
pub struct Pager {
    file: File,  // Arquivo do banco de dados
}

impl Pager {
    /// Abre um banco de dados existente
    /// 
    /// # Arguments
    /// * `db_name` - Nome do banco (sem extensão)
    pub fn new(db_name: &str) -> Self {
        let file = open_database(db_name);
        Pager { file }
    }

    /// Escreve dados em uma posição específica do arquivo
    /// 
    /// # Arguments
    /// * `offset` - Posição em bytes desde o início do arquivo
    /// * `data` - Bytes a serem escritos
    /// 
    /// # Returns
    /// * `Ok(())` se escrita bem-sucedida
    /// * `Err(e)` se erro de I/O
    pub fn write_at(&mut self, offset: u64, data: &[u8]) -> Result<()> {
        self.file.seek(SeekFrom::Start(offset))?;
        self.file.write_all(data)?;
        Ok(())
    }

    /// Lê dados de uma posição específica do arquivo
    /// 
    /// # Arguments
    /// * `offset` - Posição em bytes desde o início do arquivo
    /// * `length` - Número de bytes a ler
    /// 
    /// # Returns
    /// * `Ok(Vec<u8>)` - Bytes lidos
    /// * `Err(e)` - Se erro de I/O ou EOF inesperado
    pub fn read_at(&mut self, offset: u64, length: usize) -> Result<Vec<u8>> {
        self.file.seek(SeekFrom::Start(offset))?;
        let mut buffer = vec![0; length];
        self.file.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    /// Obtém a posição do final do arquivo
    /// 
    /// Útil para operações append-only (escrever no final)
    /// 
    /// # Returns
    /// * `Ok(u64)` - Offset do final do arquivo
    /// * `Err(e)` - Se erro de I/O
    pub fn get_end_offset(&mut self) -> Result<u64> {
        self.file.seek(SeekFrom::End(0))
    }

    /// Atualiza o offset da raiz da B-Tree no cabeçalho do arquivo
    /// 
    /// # Arguments
    /// * `root_offset` - Novo offset da raiz (8 bytes big-endian)
    /// 
    /// # Note
    /// O offset da raiz é sempre armazenado nos primeiros 8 bytes do arquivo
    pub fn update_root_offset(&mut self, root_offset: &[u8;8]) -> Result<()> {
        self.write_at(0, root_offset)
    }
}