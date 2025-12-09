use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use std::rc::Rc;

use super::button::Button;

use crate::App;

pub fn database_listing(frame: &mut Frame, app: &mut App, area: Rc<[Rect]>) {
    let mut options = app.databases.to_owned();
    options.push("Voltar".to_string());

    // Define layout
    let options_list = Layout::default()
        .direction(Direction::Vertical)
        .constraints(options.iter().map(|_| Constraint::Length(1)))
        .split(area[1]);

    // Initialize options button list
    let buttons = options
        .iter()
        .map(|name| Button::new(name.to_string(), false));

    // Render menu
    for (_i, mut btn) in buttons.enumerate() {
        if _i == app.option_highlighted.into() {
            btn.toggle_highlight();
        }

        let option_p = btn.get_paragraph();
        frame.render_widget(option_p, options_list[_i]);
    }
}
