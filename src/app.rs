use crate::connector::Connector;
use crate::editor::Editor;
use crate::tui;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::Rect,
    prelude::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, Paragraph, Widget, Wrap},
    Frame,
};
use std::{fmt, io};

#[derive(Debug, Default)]
pub enum Mode {
    #[default]
    Normal,
    Insert,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Mode::Normal => write!(f, "NORMAL"),
            Mode::Insert => write!(f, "INSERT"),
        }
    }
}

#[derive(Debug, Default)]
pub struct App {
    connector: Connector,
    editor: Editor,
    exit: bool,
    history: Vec<String>,
    mode: Mode,
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;

            self.handle_events()?;
        }

        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_input(&mut self, c: char) {
        self.editor.compute(c);
    }

    fn handle_delete(&mut self) {
        self.editor.delete();
    }

    fn connect(&mut self) {}

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char(c) => {
                self.handle_input(c);
            }
            KeyCode::Backspace => {
                self.handle_delete();
            }
            _ => {}
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            match self.mode {
                Mode::Normal => match key.code {
                    KeyCode::Char('q') => self.exit(),
                    KeyCode::Char('i') => {
                        self.mode = Mode::Insert;
                    }
                    _ => {}
                },
                Mode::Insert if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Esc => {
                        self.mode = Mode::Normal;
                    }
                    _ => self.handle_key_event(key),
                },
                _ => {}
            }
        };

        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let header = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(10), Constraint::Percentage(90)])
            .split(area);

        let toolbar = Block::new().borders(Borders::ALL).title("Actions");

        Paragraph::new(self.mode.to_string())
            .block(toolbar)
            .render(header[0], buf);

        let body = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(header[1]);

        let tables = Block::new().borders(Borders::ALL).title("Tables");

        List::new(vec![
            "table1", "table2", "table3", "table4", "table5", "table6",
        ])
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .block(tables)
        .render(body[0], buf);

        let editor = Block::new().borders(Borders::ALL).title("Editor");

        Paragraph::new(self.editor.content.as_str())
            .wrap(Wrap { trim: true })
            .block(editor)
            .render(body[1], buf);
    }
}
