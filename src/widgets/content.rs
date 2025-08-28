use rand::Rng;

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};

use palette::{FromColor, Hsv, RgbHue, Srgb};

#[derive(Clone, Copy, Debug)]
pub struct ColorBlock {
    pub block_id: usize,

    pub hsv: Hsv,

    pub selected: bool,
    pub locked: bool,
}

impl ColorBlock {
    pub fn new(block_id: usize, hue: f32, sat: f32, val: f32) -> Self {
        let hue = RgbHue::from_degrees(hue);
        let hsv: Hsv = Hsv::new(hue, sat, val);

        Self {
            block_id: block_id,

            hsv: hsv,

            selected: false,
            locked: false,
        }
    }

    pub fn generate_random_color(&mut self) {
        let mut rng = rand::rng();
        let hue = rng.random_range(0..360);
        let sat = rng.random_range(50..90); // MIGHT GONNA EDIT THESE LATER
        let val = rng.random_range(50..90);

        self.change_color(hue as f32, sat as f32 / 100.0, val as f32 / 100.0);
    }

    pub fn change_color(&mut self, hue: f32, sat: f32, val: f32) {
        let new_hue = RgbHue::from_degrees(hue);
        let hsv: Hsv = Hsv::new(new_hue, sat, val);

        self.hsv = hsv;
    }
}

impl Widget for ColorBlock {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let rgb: Srgb<f32> = Srgb::from_color(self.hsv);

        let red = (rgb.red * 255.0).round() as u8;
        let green = (rgb.green * 255.0).round() as u8;
        let blue = (rgb.blue * 255.0).round() as u8;

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

            let mut block = self.color_blocks[index].unwrap();
            if index == self.selected_block_id {
                block.selected = true;
            }

            block.render(layout[index], buf);
        }
    }
}
