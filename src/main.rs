pub mod btree;
pub mod pager;
pub mod records;
pub mod kvdb;
mod header;

use std::env;
use kvdb::KVDB;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        println!("Uso: {} <comando> <chave> [arquivo]", args[0]);
        println!("Comandos:");
        println!("  put <chave> <arquivo>   - Armazena o arquivo");
        println!("  get <chave>             - Recupera o arquivo");
        println!("  delete <chave>          - Remove o arquivo");
        return Ok(());
    }
    
    let command = &args[1];
    let key = &args[2];
    
    // Nome fixo do banco de dados
    let db_filename = "database.kvdb";
    
    match command.as_str() {
        "put" => {
            if args.len() < 4 {
                println!("Erro: Comando 'put' requer um arquivo");
                return Ok(());
            }
            let file_path = &args[3];
            
            // Verifica se o arquivo existe
            if !std::path::Path::new(file_path).exists() {
                println!("Erro: Arquivo '{}' não existe", file_path);
                return Ok(());
            }
            
            // Abre ou cria o banco de dados
            let mut db = if std::path::Path::new(db_filename).exists() {
                KVDB::open(db_filename)?
            } else {
                KVDB::create_new(db_filename)?
            };
            
            // Armazena o arquivo
            match db.put(key, file_path) {
                Ok(_) => println!("Arquivo '{}' armazenado com chave '{}'", file_path, key),
                Err(e) => println!("Erro ao armazenar: {}", e),
            }
        }
        
        "get" => {
            // Abre o banco de dados
            match KVDB::open(db_filename) {
                Ok(mut db) => {
                    // Recupera os dados
                    match db.get(key) {
                        Ok(Some(data)) => {
                            // Escreve para um arquivo
                            let output_filename = format!("{}.restored", key);
                            std::fs::write(&output_filename, &data)?;
                            println!("Arquivo recuperado como '{}'", output_filename);
                        }
                        Ok(None) => println!("Chave '{}' não encontrada", key),
                        Err(e) => println!("Erro ao recuperar: {}", e),
                    }
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::NotFound {
                        println!("Banco de dados '{}' não encontrado", db_filename);
                    } else {
                        println!("Erro ao abrir banco de dados: {}", e);
                    }
                }
            }
        }
        
        "delete" => {
            // Abre o banco de dados
            match KVDB::open(db_filename) {
                Ok(mut db) => {
                    // Remove a chave
                    match db.delete(key) {
                        Ok(_) => println!("Chave '{}' removida", key),
                        Err(e) => println!("Erro ao remover: {}", e),
                    }
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::NotFound {
                        println!("Banco de dados '{}' não encontrado", db_filename);
                    } else {
                        println!("Erro ao abrir banco de dados: {}", e);
                    }
                }
            }
        }
        
        _ => {
            println!("Comando desconhecido: {}", command);
        }
    }
    
    Ok(())
}