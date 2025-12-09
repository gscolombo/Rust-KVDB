use ratatui::{style::{Color, Style}, text::Text, widgets::Paragraph};

pub struct Button {
    name: String,
    highlight: bool,
}

impl Button {
    pub fn new(name: String, highlight: bool) -> Button {
        Button {
            name: name,
            highlight: highlight,
        }
    }

    pub fn toggle_highlight(&mut self) {
        self.highlight = !self.highlight;
    }

    pub fn get_paragraph(&mut self) -> Paragraph<'_> {
        Paragraph::new(Text::styled(
            self.name.as_str(),
            if !self.highlight {
                Style::default().fg(Color::Red)
            } else {
                Style::default().fg(Color::LightRed).bg(Color::Red)
            },
        ))
    }
}