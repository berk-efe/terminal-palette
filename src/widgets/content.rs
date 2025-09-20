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

pub fn hex2rgb(hex: &str) -> (u8, u8, u8) {
    let mut hex_owned = hex.to_string();
    hex_owned.push_str("000000");
    let padded = &hex_owned[..6];

    let r = u8::from_str_radix(&padded[0..2], 16).unwrap();
    let g = u8::from_str_radix(&padded[2..4], 16).unwrap();
    let b = u8::from_str_radix(&padded[4..6], 16).unwrap();

    (r, g, b)
}

pub fn rgb2hsv(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    // Hue
    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    let h = if h < 0.0 { h + 360.0 } else { h };

    // Saturation
    let s = if max == 0.0 { 0.0 } else { delta / max };

    // Value
    let v = max;

    (h, s, v)
}

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

    pub fn get_rgb_values(&self) -> (u8, u8, u8) {
        let rgb: Srgb<f32> = Srgb::from_color(self.hsv);

        let red = (rgb.red * 255.0).round() as u8;
        let green = (rgb.green * 255.0).round() as u8;
        let blue = (rgb.blue * 255.0).round() as u8;

        return (red, green, blue);
    }

    pub fn get_hsv_values(&self) -> (f32, f32, f32) {
        let hue: f32 = self.hsv.hue.into_raw_degrees();
        let saturation: f32 = self.hsv.saturation;
        let value: f32 = self.hsv.value;

        return (hue, saturation, value);
    }

    pub fn get_hex(&self) -> String {
        let (r, g, b) = self.get_rgb_values();
        format!("#{r:02X}{g:02X}{b:02X}")
    }

    pub fn get_avg_hue(blocks: Vec<Option<ColorBlock>>) -> f32 {
        let mut hue_as_deg: f32 = 0.0;

        for block in blocks.iter() {
            let block = block.unwrap();

            hue_as_deg += block.hsv.hue.into_degrees() as f32;
        }

        //return
        hue_as_deg / blocks.len() as f32
    }
}

impl Widget for ColorBlock {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let whole = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(1), Constraint::Fill(1)])
            .split(area);

        let mut padding = Padding::new(0, 0, whole[1].height / 2, 0);
        let selected_padding = Padding::new(0, 0, whole[1].height / 2 - 1, 0);

        let (hue, saturation, value) = self.get_hsv_values();
        let (red, green, blue) = self.get_rgb_values();

        let color = Color::Rgb(red, green, blue);

        if self.selected {
            padding = selected_padding;
        }

        let mut lock_indicator_color: Color = Color::Rgb(2, 48, 32);

        let mut lock_indicator_label = String::from("UNLOCKED");

        if self.locked {
            lock_indicator_color = Color::Rgb(139, 0, 0);
            lock_indicator_label = String::from("LOCKED");
        }

        let lock_indicator_block = Block::default()
            .borders(Borders::NONE)
            .bg(lock_indicator_color);

        let mut block = Block::default()
            .borders(Borders::NONE)
            .padding(padding)
            .bg(color);

        let selected_block = Block::default()
            .borders(Borders::ALL)
            .border_set(border::DOUBLE)
            .padding(padding)
            .bg(color);

        if self.selected {
            block = selected_block;
        }

        Paragraph::new(vec![
            Line::from(format!("HSV: {hue}, {saturation}, {value}")),
            Line::from(format!("RGB: {red}, {green}, {blue}")),
            Line::from(self.get_hex()),
            Line::from(""),
            Line::from(format!("ID: {}", self.block_id)),
        ])
        .block(block)
        .alignment(Alignment::Center)
        .render(whole[1], buf);

        Paragraph::new(Line::from(lock_indicator_label))
            .block(lock_indicator_block)
            .alignment(Alignment::Center)
            .render(whole[0], buf);
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
        let block_count = self.color_blocks.iter().filter(|b| b.is_some()).count();

        let constraints: Vec<Constraint> = vec![Constraint::Fill(1); block_count];

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(area);

        for (idx, block) in self
            .color_blocks
            .iter_mut()
            .filter_map(|maybe| maybe.as_mut())
            .enumerate()
        {
            // Mark selection
            block.selected = idx == self.selected_block_id;

            // Render into its packed layout slot
            block.render(layout[idx], buf);
        }
    }
}
