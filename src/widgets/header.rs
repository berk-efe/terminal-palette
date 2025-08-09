use ratatui::{
    buffer::Buffer,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
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

        let block = Block::bordered().border_set(border::THICK);

        Paragraph::new(title)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
