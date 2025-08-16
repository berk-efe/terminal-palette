use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};

#[derive(Clone, Copy, Debug)]
pub struct ColorBlock {
    pub block_id: usize,

    pub red: u8,
    pub green: u8,
    pub blue: u8,

    pub selected: bool,
    pub locked: bool,
}

impl ColorBlock {
    pub fn new(block_id: usize, red: u8, green: u8, blue: u8) -> Self {
        Self {
            block_id: block_id,

            red: red,
            green: green,
            blue: blue,

            selected: false,
            locked: false,
        }
    }
}

impl Widget for ColorBlock {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (red, green, blue) = (self.red, self.green, self.blue);

        let mut padding = Padding::new(0, 0, area.height / 2, 0);
        let selected_padding = Padding::new(0, 0, area.height / 2 - 1, 0);

        let color = Color::Rgb(red, green, blue);

        if self.selected {
            padding = selected_padding;
        }

        let mut block = Block::default()
            .borders(Borders::NONE)
            .padding(padding)
            .bg(color);

        let selected_block = Block::default()
            .borders(Borders::ALL)
            .border_set(border::DOUBLE)
            .padding(padding)
            .bg(color);

        let mut locked_text = String::new();
        if self.locked {
            locked_text = String::from("Locked")
        } else {
            locked_text = String::from("Unlocked")
        }

        if self.selected {
            block = selected_block;
        }

        Paragraph::new(vec![
            Line::from(format!("RGB: {red}, {green}, {blue}")),
            Line::from(""),
            Line::from(locked_text),
            Line::from(format!("(toggle with ALT+{})", self.block_id)),
            Line::from(""),
            Line::from(format!("ID: {}", self.block_id)),
        ])
        .block(block)
        .alignment(Alignment::Center)
        .render(area, buf);
    }
}

pub struct MainContent {
    pub color_blocks: [Option<ColorBlock>; 9],
    pub selected_block_id: usize,
}

impl MainContent {
    pub fn new(color_blocks: [Option<ColorBlock>; 9], selected_block_id: usize) -> Self {
        Self {
            color_blocks: color_blocks,
            selected_block_id: selected_block_id,
        }
    }
}

impl Widget for &mut MainContent {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block_count = self
            .color_blocks
            .iter()
            .filter(|block| block.is_some())
            .count();
        let mut constraints: Vec<Constraint> = Vec::new();

        for _ in 1..block_count + 1 {
            constraints.push(Constraint::Fill(1));
        }

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(area);

        for i in 1..block_count + 1 {
            let index = i - 1;

            let mut _block = self.color_blocks[index].unwrap();
            if index == self.selected_block_id {
                _block.selected = true;
            }

            _block.render(layout[index], buf);
        }
    }
}
