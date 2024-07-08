use std::time::Duration;

use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::Paragraph,
    Frame,
};

use crate::terminal;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const FRAMES_PER_SECOND: f64 = 60.0;

pub struct App {
    pub should_exit: bool,
    pub name: String,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            should_exit: false,
            name: "World".to_string(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut terminal = terminal::init()?;
        while !self.should_exit {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
        }
        terminal::restore()?;
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let [header, body, footer] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(frame.size());

        self.render_header(frame, header);
        self.render_body(frame, body);
        self.render_footer(frame, footer);
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let text = format!("{} v{}", NAME, VERSION);
        frame.render_widget(Line::raw(text).reversed(), area);
    }

    fn render_body(&self, frame: &mut Frame, area: Rect) {
        let text = format!("Hello, {}!", self.name);
        let area = Rect::new(
            area.width / 2 - text.len() as u16 / 2,
            area.height / 2,
            text.len() as u16,
            1,
        );
        frame.render_widget(Paragraph::new(text), area);
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let text = "Press 'q' to quit";
        frame.render_widget(Line::raw(text).reversed(), area);
    }

    fn handle_events(&mut self) -> Result<()> {
        let timeout = Duration::from_secs_f64(1.0 / FRAMES_PER_SECOND);
        if !event::poll(timeout)? {
            return Ok(());
        }
        match event::read()? {
            Event::Key(event) if event.kind == KeyEventKind::Press => self.handle_key_event(event),
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('q') | KeyCode::Esc => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.should_exit = true;
    }
}
