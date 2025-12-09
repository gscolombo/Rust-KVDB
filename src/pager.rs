// Atualize pager.rs
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom, Result};
use crate::header::DatabaseHeader;

pub struct Pager {
    file: File,
    header: DatabaseHeader,
}

impl Pager {
    pub fn new(filename: &str) -> Self {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filename)
            .expect("Não foi possível abrir o arquivo do banco de dados");

        // Se o arquivo estiver vazio, escreve o cabeçalho
        if file.metadata().unwrap().len() == 0 {
            let header = DatabaseHeader::new();
            let header_data = header.to_bytes();
            
            // Garante espaço para o cabeçalho (primeira página)
            let mut page = vec![0u8; 4096];
            page[..header_data.len()].copy_from_slice(&header_data);
            
            file.write_all(&page).unwrap();
        }

        // Lê o cabeçalho
        let mut header_buf = vec![0u8; 1024]; // Tamanho suficiente para o cabeçalho
        file.seek(SeekFrom::Start(0)).unwrap();
        file.read_exact(&mut header_buf).unwrap();
        
        let header = DatabaseHeader::from_bytes(&header_buf);

        Pager { file, header }
    }

    pub fn get_root_offset(&self) -> Option<u64> {
        self.header.root_offset
    }
    
    pub fn set_root_offset(&mut self, offset: u64) -> Result<()> {
        self.header.root_offset = Some(offset);
        self.write_header()
    }
    
    fn write_header(&mut self) -> Result<()> {
        self.file.seek(SeekFrom::Start(0))?;
        let header_data = self.header.to_bytes();
        self.file.write_all(&header_data)?;
        Ok(())
    }

    pub fn write_at(&mut self, offset: u64, data: &[u8]) -> Result<()> {
        self.file.seek(SeekFrom::Start(offset))?;
        self.file.write_all(data)?;
        Ok(())
    }

    pub fn read_at(&mut self, offset: u64, length: usize) -> Result<Vec<u8>> {
        self.file.seek(SeekFrom::Start(offset))?;
        let mut buffer = vec![0; length];
        self.file.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    pub fn allocate_page(&mut self) -> Result<u64> {
        // Aloca no final do arquivo
        let offset = self.file.seek(SeekFrom::End(0))?;
        
        // Preenche a página com zeros
        let page = vec![0u8; 4096];
        self.file.write_all(&page)?;
        
        Ok(offset)
    }
}