use std::io;

use rand::Rng;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    widgets::Widget,
};

use crate::widgets::content::{ColorBlock, MainContent};
use crate::widgets::header::Header;
use crate::widgets::popup::Popup;
use crate::widgets::status_bar::StatusBar;

#[derive(Debug, PartialEq)]
pub enum CurrentPage {
    Main,
    Settings,
}

#[derive(Debug)]
pub struct App {
    pub counter: i8,

    pub current_page: CurrentPage,

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
        match self.current_page {
            CurrentPage::Main => match (key_event.code, key_event.modifiers) {
                (KeyCode::Char('q'), _) => self.exit(),
                (KeyCode::Left, _) => self.decrement_counter(),
                (KeyCode::Right, _) => self.increment_counter(),

                (KeyCode::Char('c'), _) => self.current_page = CurrentPage::Settings,

                (KeyCode::Char(c), KeyModifiers::ALT) if ('1'..='9').contains(&c) => {
                    let num = c.to_digit(10).unwrap() as usize;
                    self.toggle_lock(num);
                }

                (KeyCode::Char(' '), _) => self.generate_analogous(),

                _ => {}
            },
            CurrentPage::Settings => match (key_event.code, key_event.modifiers) {
                (KeyCode::Char('c'), _) | (KeyCode::Char('q'), _) => {
                    self.current_page = CurrentPage::Main
                }
                _ => {}
            },
        }
    }

    fn generate_analogous(&mut self) {
        fn generate_random_color(block: &mut ColorBlock) {
            let mut rng = rand::rng();
            block.red = rng.random_range(0..255);
            block.green = rng.random_range(0..255);
            block.blue = rng.random_range(0..255);
        }

        let mut rng = rand::rng();

        let locked_blocks: Vec<Option<ColorBlock>> = self
            .color_blocks
            .iter()
            .filter(|block| block.is_some())
            .filter(|block| block.unwrap().locked)
            .cloned()
            .collect();

        let mut last_red: u8 = 0;
        let mut last_green: u8 = 0;
        let mut last_blue: u8 = 0;

        let rand_rate = 20;

        if locked_blocks.len() > 0 {
            let mut _red: u8 = 0;
            let mut _green: u8 = 0;
            let mut _blue: u8 = 0;

            for block in locked_blocks.iter() {
                let block = block.unwrap();
                _red = (_red as u16 + block.red as u16).clamp(1, 255) as u8;
                _green = (_green as u16 + block.green as u16).clamp(1, 255) as u8;
                _blue = (_blue as u16 + block.blue as u16).clamp(1, 255) as u8;
            }

            last_red = _red / locked_blocks.len() as u8;
            last_green = _green / locked_blocks.len() as u8;
            last_blue = _blue / locked_blocks.len() as u8;

            for block in self.color_blocks.iter_mut() {
                if let Some(color_block) = block {
                    if !color_block.locked {
                        let randomness: i8 = rng.random_range(-rand_rate..rand_rate);
                        color_block.red =
                            (last_red as i16 + randomness as i16).clamp(15, 245) as u8;
                        last_red = color_block.red;

                        color_block.green =
                            (last_green as i16 + randomness as i16).clamp(15, 245) as u8;
                        last_green = color_block.green;

                        color_block.blue =
                            (last_blue as i16 + randomness as i16).clamp(15, 245) as u8;
                        last_blue = color_block.blue;
                    }
                }
            }
        } else {
            for (i, block) in self.color_blocks.iter_mut().enumerate() {
                if let Some(color_block) = block {
                    if i == 0 {
                        generate_random_color(color_block);
                        last_red = color_block.red;
                        last_green = color_block.green;
                        last_blue = color_block.blue;
                    } else {
                        let randomness: i8 = rng.random_range(-rand_rate..rand_rate);
                        color_block.red =
                            (last_red as i16 + randomness as i16).clamp(15, 245) as u8;
                        last_red = color_block.red;
                        color_block.green =
                            (last_green as i16 + randomness as i16).clamp(15, 245) as u8;
                        last_green = color_block.green;
                        color_block.blue =
                            (last_blue as i16 + randomness as i16).clamp(15, 245) as u8;
                        last_blue = color_block.blue;
                    }
                }
            }
        }
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

            current_page: CurrentPage::Main,

            title: " Color Palette!!!!! ",
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

        // SETTINGS POPUP
        let popup_area = Rect {
            x: area.width / 4,
            y: area.height / 3,
            width: area.width / 2,
            height: area.height / 3,
        };

        if self.current_page == CurrentPage::Settings {
            let popup = Popup::default()
                .content("Hello world")
                .style(Style::new().yellow())
                .title("Popup!")
                .title_style(Style::new().white().bold())
                .border_style(Style::new().red());
            popup.render(popup_area, buf);
        }
    }
}
