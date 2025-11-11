mod db;

use db::create_database;

use crate::app::db::list_databases;

pub enum CurrentScreen {
    Main(MainMenu),
    DatabaseList,
}

pub enum MainMenu {
    OptionsList,
    CreateDb,
    SuccessMessage,
    FailureMessage,
}

pub struct App {
    pub input: String,
    pub loaded_db: Option<String>,
    pub databases: Vec<String>,
    pub current_screen: CurrentScreen,
    pub option_highlighted: u8,
}

impl App {
    pub fn new() -> App {
        let mut app = App {
            input: String::new(),
            loaded_db: None,
            databases: Vec::new(),
            current_screen: CurrentScreen::Main(MainMenu::OptionsList),
            option_highlighted: 0,
        };

        app.list_databases();
        app
    }

    pub fn create_database(&mut self) -> Result<(), std::io::Error> {
        return create_database(&self.input.to_string());
    }

    pub fn list_databases(&mut self) {
        self.databases = list_databases();
    }
}
