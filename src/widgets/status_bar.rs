use ratatui::{
    layout::Alignment,
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, Padding, Paragraph, Widget},
};

#[derive(Default, Debug)]
pub struct StatusBar {
    pub message: &'static str,
}

impl Widget for &StatusBar {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let block = Block::default()
            .bg(Color::Black)
            .padding(Padding::new(0, 0, 1, 1));

        Paragraph::new(Line::from(self.message))
            .alignment(Alignment::Left)
            .block(block)
            .render(area, buf);
    }
}
