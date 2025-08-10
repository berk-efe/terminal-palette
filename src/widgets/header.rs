use ratatui::{
    buffer::Buffer,
    style::{Color, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Padding, Paragraph, Widget},
};

pub struct Header {
    pub title: &'static str,
}

impl Header {
    pub fn new(title: &'static str) -> Self {
        Self { title: title }
    }
}

impl Widget for &Header {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut Buffer) {
        let title = Line::from(self.title.bold());

        let block = Block::default()
            .bg(Color::Rgb(36, 51, 66))
            .padding(Padding::new(0, 0, 1, 1));

        Paragraph::new(title)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
