use std::io;

use rand::Rng;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Widget,
};

use crate::widgets::content::{ColorBlock, MainContent};
use crate::widgets::header::Header;
use crate::widgets::status_bar::StatusBar;

#[derive(Debug)]
pub struct App {
    pub counter: i8,

    pub title: &'static str,
    pub color_block_count: usize,

    pub color_blocks: [Option<ColorBlock>; 9],
    pub selected_block_id: usize,

    pub status_bar_msg: &'static str,

    pub exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match (key_event.code, key_event.modifiers) {
            (KeyCode::Char('q'), _) => self.exit(),
            (KeyCode::Left, _) => self.decrement_counter(),
            (KeyCode::Right, _) => self.increment_counter(),

            (KeyCode::Char(c), KeyModifiers::ALT) if ('1'..='9').contains(&c) => {
                let num = c.to_digit(10).unwrap() as usize;
                self.toggle_lock(num);
            }

            (KeyCode::Char(' '), _) => self.generate_colors(),

            _ => {}
        }
    }

    fn generate_colors(&mut self) {
        fn generate_random_color(block: &mut ColorBlock) {
            let mut rng = rand::rng();
            block.red = rng.random_range(0..255);
            block.green = rng.random_range(0..255);
            block.blue = rng.random_range(0..255);
        }

        self.color_blocks
            .iter_mut()
            .filter(|block| block.is_some())
            .filter(|block| !block.unwrap().locked)
            .for_each(|block| generate_random_color(block.as_mut().unwrap()));
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        self.selected_block_id = self
            .selected_block_id
            .saturating_add(1)
            .clamp(0, self.color_block_count - 1);
    }

    fn decrement_counter(&mut self) {
        self.selected_block_id = self
            .selected_block_id
            .saturating_sub(1)
            .clamp(0, self.color_block_count - 1);
    }

    fn toggle_lock(&mut self, id: usize) {
        self.color_blocks[id - 1].as_mut().map(|color_block| {
            color_block.locked = !color_block.locked;
        });
    }
}

impl Default for App {
    fn default() -> Self {
        let color_block_count: usize = 5;
        let mut color_blocks: [Option<ColorBlock>; 9] = [None; 9];

        for i in 1..color_block_count + 1 {
            color_blocks[i - 1] = Some(ColorBlock::new(i, 0, 0, 0));
        }

        Self {
            counter: 0,

            title: " Color Palette ",
            color_block_count: color_block_count,
            selected_block_id: 0,

            color_blocks: color_blocks,

            status_bar_msg: "",

            exit: false,
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // SELECTED BLOCK
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(3),
                Constraint::Fill(1),
                Constraint::Length(3),
            ])
            .split(area);

        let (header_area, main_area, footer_area) = (layout[0], layout[1], layout[2]);

        let header = Header::new(self.title);
        header.render(header_area, buf);

        let mut main_content = MainContent::new(self.color_blocks, self.selected_block_id);
        main_content.render(main_area, buf);

        let status_bar = StatusBar::default();
        status_bar.render(footer_area, buf);
    }
}
