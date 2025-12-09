mod menus;
mod ui;
mod db_loaded_event_loop;
mod main_event_loop;
mod shared;

use ratatui::Terminal;
use ratatui::crossterm::event::{self, Event, KeyEvent};
use ratatui::prelude::Backend;
use std::io::*;

use crate::app::{App, CurrentScreen, DatabasePrompt, MainMenu};
use crate::cli::menus::{DatabaseListingOptions, database_list};

use ui::ui;

use main_event_loop::main_menu_event_loop;
use db_loaded_event_loop::database_loaded_event_loop;


fn event_loop(key: KeyEvent, app: &mut App) -> Option<Result<bool>> {
    match app.current_screen {
        CurrentScreen::Main(_) => { 
            if main_menu_event_loop(key, app) {
                return Some(Ok(true));
            }
         },
        CurrentScreen::DatabaseList => match database_list(key, app) {
            Some(DatabaseListingOptions::ChooseDb) => {
                app.current_screen = CurrentScreen::DatabaseLoaded(DatabasePrompt::SelectCommand);
            }
            Some(DatabaseListingOptions::Exit) => {
                app.current_screen = CurrentScreen::Main(MainMenu::OptionsList);
            }
            None => {}
        },
        CurrentScreen::DatabaseLoaded(_) => {
            database_loaded_event_loop(key, app);
        },
    }

    None
}

pub fn run<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<bool> {
    loop {
        terminal.draw(|frame| ui(frame, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match event_loop(key, app) {
                Some(res) => return res,
                None => continue,
            }
        }
    }
}
