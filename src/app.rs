use core::fmt;
use crate::pager::Pager;

use ratatui::widgets::ScrollbarState;

use crate::{btree::BTree, db::list_databases};

/// Representa a tela atual da aplicação
/// 
/// A aplicação tem três estados principais: tela inicial, listagem de bancos,
/// e banco carregado com operações
pub enum CurrentScreen {
    Main(MainMenu),                    // Tela principal com menu
    DatabaseList,                     // Lista de bancos de dados disponíveis
    DatabaseLoaded(DatabasePrompt),   // Banco carregado com prompt de comandos
}

impl Default for CurrentScreen {
    /// Define a tela inicial padrão como o menu principal
    fn default() -> Self {
        CurrentScreen::Main(MainMenu::OptionsList)
    }
}

/// Subestados da tela principal
pub enum MainMenu {
    OptionsList,      // Apresenta as opções do menu principal
    CreateDb,         // Pop-up para criar novo banco de dados
    SuccessMessage,   // Mensagem de sucesso após operação
    // FailureMessage, // (Comentado) Possível estado para mensagens de erro
}

/// Subestados quando um banco de dados está carregado
pub enum DatabasePrompt {
    SelectCommand,   // Seleção de comando (SEARCH, INSERT, DELETE, CLOSE)
    UserInput,       // Entrada de dados do usuário
    ResultView,      // Visualização de resultados da busca
    SuccessMessage,  // Mensagem de sucesso
    FailureMessage,  // Mensagem de falha
}

/// Comandos disponíveis para operar no banco de dados
pub enum DatabaseCommands {
    SEARCH,  // Buscar um valor por chave
    INSERT,  // Inserir novo par chave-valor
    DELETE,  // Remover par chave-valor
    CLOSE,   // Fechar o banco de dados
}

impl fmt::Display for DatabaseCommands {
    /// Implementa formatação para exibição dos comandos
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::SEARCH => write!(f, "SEARCH"),
            Self::INSERT => write!(f, "INSERT"),
            Self::DELETE => write!(f, "DELETE"),
            Self::CLOSE => write!(f, "CLOSE"),
        }
    }
}

#[derive(Default)]
/// Estrutura principal que mantém o estado da aplicação
/// 
/// Utiliza o padrão de estado (state pattern) para gerenciar diferentes
/// telas e funcionalidades da interface
pub struct App {
    pub input: String,               // Entrada atual do usuário
    pub loaded_db: Option<Pager>,    // Referência ao banco carregado (se houver)
    pub databases: Vec<String>,      // Lista de bancos disponíveis
    pub current_screen: CurrentScreen, // Tela atual
    pub option_highlighted: u8,      // Índice da opção destacada no menu
    pub db_command: Option<DatabaseCommands>, // Comando selecionado
    pub index: BTree,                // Índice B-Tree do banco
    pub search_result: String,       // Resultado da última busca
    // pub failure_message: String,   // (Comentado) Mensagem de erro
    pub vertical_scroll_state: ScrollbarState, // Estado da barra de rolagem
    pub vertical_scroll: u16,        // Posição de rolagem vertical
    pub line_count: usize            // Contagem de linhas para rolagem
}

impl App {
    /// Cria uma nova instância da aplicação
    /// 
    /// Inicializa com estado padrão e carrega a lista de bancos disponíveis
    pub fn new() -> App {
        let mut app = App::default();
        app.fetch_databases();  // Carrega bancos ao inicializar
        app
    }

    /// Atualiza a lista de bancos de dados disponíveis
    /// 
    /// Lê o diretório `./databases` e popula o vetor `databases`
    pub fn fetch_databases(&mut self) {
        self.databases = list_databases();
    }
}