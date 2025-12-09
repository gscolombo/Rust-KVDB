use core::fmt;
use crate::pager::Pager;

use ratatui::widgets::ScrollbarState;

use crate::{btree::BTree, db::list_databases};

// Representa a tela atual da aplicação
pub enum CurrentScreen {
    Main(MainMenu),
    DatabaseList, // Tela com listagem de bancos de dados disponíveis para carregamento
    DatabaseLoaded(DatabasePrompt), // Tela para selecionar e executar um comando no banco de dados
}

impl Default for CurrentScreen {
    fn default() -> Self {
        CurrentScreen::Main(MainMenu::OptionsList)
    }
}

pub enum MainMenu {
    OptionsList, // Apresenta as opções do menu principal
    CreateDb,    // Pop-up para entrada de usuário com o nome de um novo banco de dados
    SuccessMessage,
    // FailureMessage,
}

pub enum DatabasePrompt {
    SelectCommand, // Apresenta os comandos disponíveis para executar no banco de dados
    UserInput,     // Pop-up para entrada de usuário com parâmetros do comando selecionado
    ResultView,    // Tela de resultados da operação de busca (SEARCH)
    SuccessMessage,
    FailureMessage,
}

pub enum DatabaseCommands {
    SEARCH,
    INSERT,
    DELETE,
    CLOSE,
}

impl fmt::Display for DatabaseCommands {
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
// Representa o estado atual da aplicação
pub struct App {
    pub input: String,
    pub loaded_db: Option<Pager>,
    pub databases: Vec<String>,
    pub current_screen: CurrentScreen,
    pub option_highlighted: u8,
    pub db_command: Option<DatabaseCommands>,
    pub index: BTree,
    pub search_result: String,
    // pub failure_message: String,
    pub vertical_scroll_state: ScrollbarState,
    pub vertical_scroll: u16,
    pub line_count: usize
}

impl App {
    pub fn new() -> App {
        let mut app = App::default();

        app.fetch_databases();
        app
    }

    // Atualiza a lista de banco de dados da aplicação
    pub fn fetch_databases(&mut self) {
        self.databases = list_databases();
    }
}
