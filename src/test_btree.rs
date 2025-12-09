use crate::btree::{BTree, Node};
use crate::pager::Pager;
use std::fs;

/// Teste básico da B-Tree em memória
fn test_btree_basic() {
    println!("=== TESTE B-TREE BÁSICO ===");
    
    // Cria uma B-Tree nova
    let mut btree = BTree::new();
    println!("B-Tree criada: {:?}", btree);
    
    // Cria um Pager temporário (em memória)
    let mut pager = Pager::new("test_btree.db");
    
    // Insere alguns valores
    println!("\nInserindo chaves...");
    
    let test_data = vec![
        ("chave1", "valor1"),
        ("chave2", "valor2"),
        ("chave3", "valor3"),
        ("chave4", "valor4"),
        ("chave5", "valor5"),
    ];
    
    for (key, value) in test_data {
        println!("Inserindo: {} -> {}", key, value);
        match btree.insert(key.to_string(), value.to_string(), &mut pager) {
            Ok(_) => println!("  ✓ Inserido com sucesso"),
            Err(e) => println!("  ✗ Erro: {}", e),
        }
    }
    
    println!("\nB-Tree após inserções: {:?}", btree);
    
    // Testa buscas
    println!("\nTestando buscas...");
    let search_keys = vec!["chave1", "chave3", "chave5", "nao_existe"];
    
    for key in search_keys {
        match btree.search(key, &mut pager) {
            Some(value) => println!("  {} encontrado: {}", key, value),
            None => println!("  {} NÃO encontrado", key),
        }
    }
    
    // Limpa arquivo de teste
    let _ = fs::remove_file("test_btree.db");
    println!("\n=== TESTE CONCLUÍDO ===");
}

/// Teste de divisão de nós (quando nó fica cheio)
fn test_btree_split() {
    println!("\n=== TESTE DE DIVISÃO DE NÓS ===");
    
    // Cria nova B-Tree
    let mut btree = BTree::new();
    let mut pager = Pager::new("test_split.db");
    
    // Inserir mais chaves que o máximo por nó (MAX_KEYS = 5)
    // Isso deve forçar divisões
    println!("Inserindo {} chaves (máximo por nó é {})...", 10, crate::btree::MAX_KEYS);
    
    for i in 1..=10 {
        let key = format!("key{:03}", i);
        let value = format!("value{:03}", i);
        
        println!("Inserindo: {} -> {}", key, value);
        if let Err(e) = btree.insert(key.clone(), value, &mut pager) {
            println!("  ✗ Erro ao inserir {}: {}", key, e);
        }
    }
    
    println!("\nEstrutura da B-Tree após múltiplas inserções:");
    println!("Raiz: {:?}", btree.root);
    
    // Verifica se todas as chaves podem ser encontradas
    println!("\nVerificando todas as chaves...");
    let mut todas_encontradas = true;
    
    for i in 1..=10 {
        let key = format!("key{:03}", i);
        match btree.search(&key, &mut pager) {
            Some(value) => println!("  ✓ {} encontrado: {}", key, value),
            None => {
                println!("  ✗ {} NÃO encontrado", key);
                todas_encontradas = false;
            }
        }
    }
    
    if todas_encontradas {
        println!("\n✅ TODAS as chaves foram encontradas!");
    } else {
        println!("\n❌ ALGUMAS chaves NÃO foram encontradas!");
    }
    
    // Testa busca por chave não existente
    println!("\nTestando chave não existente...");
    match btree.search("nao_existe_999", &mut pager) {
        Some(_) => println!("  ✗ Chave não existente foi encontrada (ERRO)"),
        None => println!("  ✓ Chave não existente não foi encontrada (CORRETO)"),
    }
    
    // Limpa arquivo
    let _ = fs::remove_file("test_split.db");
    println!("=== TESTE DE DIVISÃO CONCLUÍDO ===");
}

/// Teste de persistência (salvar e carregar)
fn test_btree_persistence() {
    println!("\n=== TESTE DE PERSISTÊNCIA ===");
    
    let filename = "test_persistence.db";
    
    // Fase 1: Criar e popular B-Tree
    println!("Fase 1: Criando e populando B-Tree...");
    let mut btree1 = BTree::new();
    let mut pager1 = Pager::new(filename);
    
    for i in 1..=5 {
        let key = format!("persist_key{}", i);
        let value = format!("persist_value{}", i);
        btree1.insert(key, value, &mut pager1).unwrap();
    }
    
    println!("B-Tree 1 criada. Raiz: {:?}", btree1.root);
    
    // IMPORTANTE: Para persistir, precisamos salvar a raiz
    // A B-Tree atual não faz isso automaticamente
    println!("⚠️  A B-Tree atual NÃO persiste a raiz automaticamente");
    println!("   (Isso será implementado na integração completa)");
    
    // Fase 2: "Recriar" B-Tree (simulação)
    println!("\nFase 2: Recriando B-Tree do mesmo arquivo...");
    
    // Na prática, precisaríamos:
    // 1. Salvar offset da raiz em local conhecido (ex: início do arquivo)
    // 2. Ao reabrir, ler offset e carregar raiz
    
    println!("Simulando recriação...");
    let btree2 = BTree::new(); // Nova instância
    let mut pager2 = Pager::new(filename); // Mesmo arquivo
    
    // Tentar buscar chaves (não vai funcionar sem persistir a raiz)
    println!("Buscando 'persist_key3' na nova instância...");
    match btree2.search("persist_key3", &mut pager2) {
        Some(v) => println!("  Encontrado: {}", v),
        None => println!("  Não encontrado (esperado, pois raiz não foi persistida)"),
    }
    
    // Limpa arquivo
    let _ = fs::remove_file(filename);
    println!("=== TESTE DE PERSISTÊNCIA CONCLUÍDO ===");
}

/// Teste com verificação de estrutura
fn test_btree_structure() {
    println!("\n=== TESTE DE ESTRUTURA DA B-TREE ===");
    
    let mut btree = BTree::new();
    let mut pager = Pager::new("test_structure.db");
    
    // Inserir dados em ordem aleatória para testar balanceamento
    let keys = vec!["m", "d", "a", "h", "t", "p", "z", "c", "b", "f"];
    
    println!("Inserindo chaves em ordem aleatória: {:?}", keys);
    
    for (i, key) in keys.iter().enumerate() {
        let value = format!("val_{}", key);
        btree.insert(key.to_string(), value, &mut pager).unwrap();
        println!("Após inserir {} ({} de {}):", key, i+1, keys.len());
        
        // Imprime estrutura após cada inserção
        if keys.len() <= 10 { // Só imprime se não for muito grande
            btree.print_structure(&mut pager);
        }
    }
    
    // Verifica integridade
    println!("\nVerificando integridade da árvore...");
    let total_keys = btree.count_keys(&mut pager);
    println!("Total de chaves na árvore: {}", total_keys);
    println!("Esperado: {}", keys.len());
    
    if total_keys == keys.len() {
        println!("✅ Contagem de chaves CORRETA!");
    } else {
        println!("❌ Contagem de chaves INCORRETA!");
    }
    
    // Limpa arquivo
    let _ = fs::remove_file("test_structure.db");
    println!("=== TESTE DE ESTRUTURA CONCLUÍDO ===");
}

/// Função principal de teste
pub fn run_all_tests() {
    println!("🚀 INICIANDO TESTES DA B-TREE 🚀");
    println!("=================================\n");
    
    test_btree_basic();
    test_btree_split();
    test_btree_structure(); 
    test_btree_persistence();
    
    println!("\n=================================");
    println!("✅ TODOS OS TESTES CONCLUÍDOS ✅");
}