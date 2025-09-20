use std::io;

use palette::Hsv;
use rand::Rng;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, ListState, Paragraph, Widget},
};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use arboard::Clipboard;

use crate::widgets::{
    content::{hex2rgb, rgb2hsv},
    status_bar::StatusBar,
};
use crate::{
    margin,
    widgets::content::{ColorBlock, MainContent},
};

pub const HEX_CHARS: [char; 22] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'A', 'B', 'C', 'D', 'E', 'F', '0', '1', '2', '3', '4', '5', '6',
    '7', '8', '9',
];

#[derive(Debug, PartialEq)]
pub enum CurrentPage {
    Main,
    TheorySelector,
    EditColor,
}

#[derive(Copy, Clone, Debug, PartialEq, EnumIter)]
pub enum ColorTheories {
    Analogous,
    Complementary,
}

pub struct App {
    pub counter: i8,

    pub clipboard: Clipboard,

    pub theory_selector_state: ListState,
    pub current_page: CurrentPage,
    pub current_color_theory: ColorTheories,

    pub title: &'static str,
    pub color_block_count: usize,

    pub color_blocks: [Option<ColorBlock>; 9],
    pub selected_block_id: usize,

    pub status_bar_msg: &'static str,

    pub edit_color_field: String,

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

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(&*self, frame.area());

        let popup_area = Rect {
            x: frame.area().width / 3,
            y: frame.area().height * 2 / 5,
            width: frame.area().width / 3,
            height: frame.area().height / 4,
        };

        if self.current_page == CurrentPage::TheorySelector {
            // SETTINGS POPUP

            let popup_list_items: Vec<ListItem> = ColorTheories::iter()
                .map(|t| ListItem::new(format!("{:?}", t)))
                .collect();

            let popup_list = List::new(popup_list_items)
                .block(
                    Block::default()
                        .title(" Select Theory ")
                        .borders(Borders::ALL)
                        .border_type(BorderType::Plain),
                )
                .highlight_symbol(">");

            frame.render_widget(Clear, popup_area);
            frame.render_stateful_widget(popup_list, popup_area, &mut self.theory_selector_state);
        } else if self.current_page == CurrentPage::EditColor {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Fill(1), Constraint::Fill(1)])
                .split(popup_area);

            let block = Block::default()
                .title(" Edit Color ")
                .borders(Borders::ALL)
                .border_type(BorderType::Plain);

            frame.render_widget(block, popup_area);

            // DOESNT WORK FOR SOME REASON,
            // let par_w_blink = Paragraph::new(Line::from(vec![
            //     Span::from(format!(" > {}", "Hello World")),
            //     Span::from("█").add_modifier(Modifier::SLOW_BLINK),
            // ]))
            // .block(block);

            let (r, g, b) = hex2rgb(&self.edit_color_field);

            let par = Paragraph::new(format!(" Enter HEX: {}", &self.edit_color_field));
            let overview = Paragraph::new(Line::from("Overview:").add_modifier(Modifier::REVERSED))
                .block(Block::new().bg(Color::Rgb(r, g, b)));

            frame.render_widget(Clear, popup_area.inner(margin!(1, 1)));
            frame.render_widget(par, layout[0].inner(margin!(1, 1)));
            frame.render_widget(overview, layout[1].inner(margin!(1, 1)));
        }
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

                (KeyCode::Char('d'), _) if self.color_block_count > 3 => self.del_block(),

                (KeyCode::Char('x'), _) => {
                    self.theory_selector_state.select_first();
                    self.current_page = CurrentPage::TheorySelector
                }

                (KeyCode::Char('z'), _) => {
                    self.current_page = CurrentPage::EditColor;
                }

                (KeyCode::Char('l'), _) => {
                    if let Some(block) = self.color_blocks[self.selected_block_id].as_mut() {
                        block.locked = !block.locked;
                    }
                }

                (KeyCode::Char('c'), _) => {
                    self.clipboard
                        .set_text(self.color_blocks[self.selected_block_id].unwrap().get_hex())
                        .unwrap();
                }

                (KeyCode::Char(c), KeyModifiers::ALT) if ('1'..='9').contains(&c) => {
                    let num = c.to_digit(10).unwrap() as usize;
                    self.toggle_lock(num);
                }

                (KeyCode::Char(' '), _) => match self.current_color_theory {
                    ColorTheories::Analogous => self.generate_analogous(),
                    ColorTheories::Complementary => self.generate_complementary(),
                },

                _ => {}
            },
            CurrentPage::TheorySelector => match (key_event.code, key_event.modifiers) {
                (KeyCode::Char('x'), _) | (KeyCode::Char('q'), _) | (KeyCode::Esc, _) => {
                    self.current_page = CurrentPage::Main
                }

                (KeyCode::Left, _) => self.theory_selector_state.select_first(),
                (KeyCode::Right, _) => self.theory_selector_state.select_last(),
                (KeyCode::Up, _) => self.theory_selector_state.select_previous(),
                (KeyCode::Down, _) => self.theory_selector_state.select_next(),

                (KeyCode::Enter, _) | (KeyCode::Char(' '), _) => {
                    if let Some(selected) = self.theory_selector_state.selected() {
                        let theories: Vec<ColorTheories> = ColorTheories::iter().collect();
                        self.current_color_theory = theories[selected];
                        self.current_page = CurrentPage::Main;
                    }
                }

                _ => {}
            },

            CurrentPage::EditColor => match (key_event.code, key_event.modifiers) {
                (KeyCode::Char('z'), _) | (KeyCode::Char('q'), _) => {
                    self.current_page = CurrentPage::Main
                }

                (KeyCode::Char(c), _)
                    if HEX_CHARS.contains(&c) && self.edit_color_field.len() < 6 =>
                {
                    self.edit_color_field.push(c);
                }

                // doesnt work gonna look later
                (KeyCode::Backspace, KeyModifiers::CONTROL) => {
                    self.edit_color_field = String::new();
                }

                (KeyCode::Backspace, _) => {
                    self.edit_color_field.pop();
                }

                (KeyCode::Enter, _) => {
                    if let Some(block) = self.color_blocks[self.selected_block_id].as_mut() {
                        let (r, g, b) = hex2rgb(&self.edit_color_field);
                        let (h, s, v) = rgb2hsv(r, g, b);
                        block.hsv = Hsv::new(h, s, v);
                        self.edit_color_field = String::new();
                    }
                }

                _ => {}
            },
        }
    }

    fn get_locked_blocks(&mut self) -> Vec<Option<ColorBlock>> {
        self.color_blocks
            .iter()
            .filter(|block| block.is_some())
            .filter(|block| block.unwrap().locked)
            .cloned()
            .collect()
    }

    fn generate_complementary(&mut self) {
        let mut rng = rand::rng();
        let locked_blocks = self.get_locked_blocks();

        let mut last_hue_as_deg: f32 = 0.0;
        let rand_rate = 60; // GONNA MAKE THIS INTO A GLOBAL CONS

        if locked_blocks.len() > 0 {
            last_hue_as_deg = ColorBlock::get_avg_hue(locked_blocks);

            for block in self.color_blocks.iter_mut() {
                if let Some(color_block) = block {
                    if !color_block.locked {
                        let randomness = rng.random_range(-rand_rate..rand_rate);

                        let new_hue_as_deg: f32 = last_hue_as_deg + 180.0 + randomness as f32;

                        color_block.change_color(
                            new_hue_as_deg,
                            color_block.hsv.saturation,
                            color_block.hsv.value,
                        );
                        last_hue_as_deg = new_hue_as_deg;
                    }
                }
            }
        } else {
            for (i, block) in self.color_blocks.iter_mut().enumerate() {
                if let Some(color_block) = block {
                    if i == 0 {
                        color_block.generate_random_color();
                        last_hue_as_deg = color_block.hsv.hue.into_degrees();
                        color_block.change_color(
                            last_hue_as_deg,
                            color_block.hsv.saturation,
                            color_block.hsv.value,
                        );
                    } else {
                        let randomness = rng.random_range(-rand_rate..rand_rate);
                        let new_hue_as_degrees: f32 = last_hue_as_deg + 180.0 + randomness as f32;
                        let new_sat: f32 = rng.random_range(50..80) as f32;
                        let new_val: f32 = rng.random_range(50..80) as f32;

                        color_block.change_color(
                            new_hue_as_degrees,
                            new_sat / 100.0,
                            new_val / 100.0,
                        );

                        last_hue_as_deg = new_hue_as_degrees;
                    }
                }
            }
        }
    }

    fn generate_analogous(&mut self) {
        let mut rng = rand::rng();

        let locked_blocks = self.get_locked_blocks();

        let mut last_hue_as_deg: f32 = 0.0;
        let rand_rate = 60;

        if locked_blocks.len() > 0 {
            last_hue_as_deg = ColorBlock::get_avg_hue(locked_blocks);

            for block in self.color_blocks.iter_mut() {
                if let Some(color_block) = block {
                    if !color_block.locked {
                        let randomness = rng.random_range(0..rand_rate);

                        let new_hue_as_deg: f32 = last_hue_as_deg + randomness as f32;

                        color_block.change_color(
                            new_hue_as_deg,
                            color_block.hsv.saturation,
                            color_block.hsv.value,
                        );

                        last_hue_as_deg = new_hue_as_deg;
                    }
                }
            }
        } else {
            for (i, block) in self.color_blocks.iter_mut().enumerate() {
                if let Some(color_block) = block {
                    if i == 0 {
                        color_block.generate_random_color();
                        last_hue_as_deg = color_block.hsv.hue.into_degrees();
                        color_block.change_color(
                            last_hue_as_deg,
                            color_block.hsv.saturation,
                            color_block.hsv.value,
                        );
                    } else {
                        let randomness = rng.random_range(0..rand_rate);
                        let new_hue_as_degrees: f32 = last_hue_as_deg + randomness as f32;
                        let new_sat: f32 = rng.random_range(50..80) as f32;
                        let new_val: f32 = rng.random_range(50..80) as f32;

                        color_block.change_color(
                            new_hue_as_degrees,
                            new_sat / 100.0,
                            new_val / 100.0,
                        );

                        last_hue_as_deg = new_hue_as_degrees;
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

    fn generate_blocks(&mut self) {}

    fn add_block(&mut self) {}

    fn del_block(&mut self) {
        self.color_blocks[self.selected_block_id] = None;
        self.color_block_count -= 1;
        self.selected_block_id = 0;
    }
}

impl Default for App {
    fn default() -> Self {
        let color_block_count: usize = 5;
        let mut color_blocks: [Option<ColorBlock>; 9] = [None; 9];

        for i in 1..color_block_count + 1 {
            color_blocks[i - 1] = Some(ColorBlock::new(i, 0.0, 0.0, 0.0));
        }

        Self {
            counter: 0,

            clipboard: Clipboard::new().unwrap(),

            theory_selector_state: ListState::default(),
            current_page: CurrentPage::Main,
            current_color_theory: ColorTheories::Analogous,

            title: " Color Palette!!!!! ",
            color_block_count: color_block_count,
            selected_block_id: 0,

            color_blocks: color_blocks,

            status_bar_msg: "",

            edit_color_field: String::new(),

            exit: false,
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // SELECTED BLOCK
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Fill(1), Constraint::Length(3)])
            .split(area);

        let (main_area, footer_area) = (layout[0], layout[1]);

        let mut main_content = MainContent::new(self.color_blocks, self.selected_block_id);
        main_content.render(main_area, buf);

        let status_bar = StatusBar::default();
        status_bar.render(footer_area, buf);
    }
}
