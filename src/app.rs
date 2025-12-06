use core::fmt;
use std::fs::File;

use crate::db::list_databases;

pub enum CurrentScreen {
    Main(MainMenu),
    DatabaseList,
    DatabaseLoaded(DatabasePrompt)
}

pub enum MainMenu {
    OptionsList,
    CreateDb,
    SuccessMessage,
    FailureMessage,
}

pub enum DatabasePrompt {
    SelectCommand,
    UserInput,
    ResultView,
    SuccessMessage,
    FailureMessage
}

pub enum DatabaseCommands {
    SEARCH,
    INSERT,
    DELETE,
    CLOSE
}

impl fmt::Display for DatabaseCommands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::SEARCH => write!(f, "SEARCH"),
            Self::INSERT => write!(f, "INSERT"),
            Self::DELETE => write!(f, "DELETE"),
            Self::CLOSE => write!(f, "CLOSE")
        }
    }
}

pub struct App {
    pub input: String,
    pub loaded_db: Option<File>,
    pub databases: Vec<String>,
    pub current_screen: CurrentScreen,
    pub option_highlighted: u8,
    pub db_command: Option<DatabaseCommands>,
}

impl App {
    pub fn new() -> App {
        let mut app = App {
            input: String::new(),
            loaded_db: None,
            databases: Vec::new(),
            current_screen: CurrentScreen::Main(MainMenu::OptionsList),
            option_highlighted: 0,
            db_command: None
        };

        app.fetch_databases();
        app
    }

    pub fn fetch_databases(&mut self) {
        self.databases = list_databases();
    }
}
