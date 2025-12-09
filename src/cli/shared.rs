use crate::app::{App, CurrentScreen};
use ratatui::crossterm::event::{KeyCode, KeyEvent};

pub fn user_input<F>(key: KeyEvent, app: &mut App, esc_screen: CurrentScreen, mut callback: F)
where
    F: FnMut(&mut App) -> (),
{
    match key.code {
        KeyCode::Char(value) => {
            app.input.push(value);
        }
        KeyCode::Backspace => {
            app.input.pop();
        }
        KeyCode::Esc => {
            app.input.clear();
            app.current_screen = esc_screen;
        }
        KeyCode::Enter if !app.input.is_empty() => {
            callback(app);
        }
        _ => {}
    }
}
