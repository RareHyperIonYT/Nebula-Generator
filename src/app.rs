use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};
use rand::Rng;

use crate::password::PasswordStrength;
use crate::ui::render_ui;

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub password: String,
    pub length: usize,
    pub use_uppercase: bool,
    pub use_numbers: bool,
    pub use_symbols: bool,
    pub strength: PasswordStrength,
}

impl Default for App {
    fn default() -> Self {
        let length = 12;
        let use_uppercase = true;
        let use_numbers = true;
        let use_symbols = true;
        let strength = PasswordStrength::from_settings(length, use_uppercase, use_numbers, use_symbols);
        let mut app = Self {
            running: false,
            password: String::new(),
            length,
            use_uppercase,
            use_numbers,
            use_symbols,
            strength,
        };
        app.generate_password();
        app
    }
}

impl App {
    pub fn new() -> Self { Self::default() }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|f| self.render(f))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render(&mut self, f: &mut Frame) {
        render_ui(self, f);
    }

    fn handle_events(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                self.handle_key(key);
            }
        }

        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.running = false,
            (_, KeyCode::Char('+') | KeyCode::Char('=')) => { self.length += 1; self.update(); },
            (_, KeyCode::Char('-')) if self.length > 1 => { self.length -= 1; self.update(); },
            (_, KeyCode::Char('u')) => { self.use_uppercase = !self.use_uppercase; self.update(); },
            (_, KeyCode::Char('n')) => { self.use_numbers = !self.use_numbers; self.update(); },
            (_, KeyCode::Char('s')) => { self.use_symbols = !self.use_symbols; self.update(); },
            (_, KeyCode::Char('g')) => self.generate_password(),
            _ => {},
        }
    }

    fn update(&mut self) {
        self.strength = PasswordStrength::from_settings(
            self.length,
            self.use_uppercase,
            self.use_numbers,
            self.use_symbols
        );
        self.generate_password();
    }

    pub fn generate_password(&mut self) {
        let mut charset = String::from("abcdefghijklmnopqrstuvwxyz");
        if self.use_uppercase { charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ"); }
        if self.use_numbers { charset.push_str("0123456789"); }
        if self.use_symbols { charset.push_str("!@#$%^&*()-_=+[]{};:'\",.<>?/\\|"); }

        let mut rng = rand::rng();
        self.password = (0..self.length)
            .map(|_| {
                let idx = rng.random_range(0..charset.len());
                charset.chars().nth(idx).unwrap()
            })
            .collect();
    }
}