use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom, Result};

pub struct Pager {
    file: File,
}

impl Pager {
    // Abre ou cria o arquivo do banco de dados
    pub fn new(filename: &str) -> Self {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filename)
            .expect("Não foi possível abrir o arquivo do banco de dados");

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
}