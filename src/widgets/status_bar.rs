use ratatui::{
    layout::Alignment,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

#[derive(Default, Debug)]
pub struct StatusBar {
    pub message: &'static str,
}

impl Widget for &StatusBar {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let block = Block::bordered().border_set(border::THICK);

        Paragraph::new(Line::from(self.message))
            .alignment(Alignment::Left)
            .block(block)
            .render(area, buf);
    }
}
