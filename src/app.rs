use crossterm::event::{Event, KeyEventKind, KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Stylize},
    text::{Line, Span},
    widgets::{Block, Padding, Paragraph, Gauge, Wrap},
    DefaultTerminal, Frame,
};

use crate::example::LabelledExample;

pub struct App {
    pub examples: Vec<LabelledExample>,
    labels: Vec<String>,
    index: usize
}

impl App {
    pub fn new(examples: Vec<LabelledExample>, labels: Vec<String>) -> Option<Self> {
        if examples.is_empty() || labels.is_empty() {
            return None;
        }

        Some(Self {
            examples,
            labels,
            index: 0
        })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;

            match crossterm::event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    match key_event.code {
                        KeyCode::Char(c) if c == 'c' && key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                            break;
                        }
                        KeyCode::Left => {
                            if self.index > 0 {
                                self.index = self.index - 1;
                            }
                        }
                        KeyCode::Right => {
                            self.index = std::cmp::min(self.examples.len() - 1, self.index + 1);
                        }
                        KeyCode::Up => {
                            let example = &mut self.examples[self.index];

                            match example.label.as_ref() {
                                None => {
                                    example.label = Some(self.labels[0].clone());
                                }
                                Some(current) => {
                                    let label_index = self.labels
                                        .iter()
                                        .position(|l| &l == &current);

                                    example.label = match label_index {
                                        None => {
                                            Some(self.labels[0].clone())
                                        }
                                        Some(i) if i < self.labels.len() - 1 => {
                                            Some(self.labels[i + 1].clone())
                                        }
                                        _ => None
                                    };
                                }
                            }
                        }
                        KeyCode::Tab => {
                            self.index = self.examples
                                .iter()
                                .position(|e| e.label.is_none())
                                .unwrap_or(self.examples.len() - 1);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        let example = &self.examples[self.index];

        let [left, right] = Layout::horizontal([
            Constraint::Percentage(75),
            Constraint::Percentage(25)
        ]).areas(frame.area());

        let [progress, top_left, bottom_left] = Layout::vertical([
            Constraint::Percentage(2),
            Constraint::Fill(1),
            Constraint::Fill(1)
        ]).areas(left);

        let [top_right, bottom_right] = Layout::vertical([
            Constraint::Length(7),
            Constraint::Fill(1)
        ]).areas(right);

        let index_frac = ((self.index + 1) as f32) / (self.examples.len() as f32);
        let progress_gauge = Gauge::default()
            .percent((index_frac * 100.0).floor() as u16);

        let text_title = format!(" Example text ({} / {}) ", self.index + 1, self.examples.len());
        let text_block = Block::bordered()
            .padding(Padding { left: 2, right: 2, top: 2, bottom: 2 })
            .title(Line::from(text_title).centered());

        let text = Paragraph::new(example.example.text.as_str())
            .wrap(Wrap { trim: true })
            .centered()
            .block(text_block);

        let ground_truth_block = Block::bordered()
            .padding(Padding { left: 1, right: 1, top: 1, bottom: 1 })
            .title(Line::from(" Ground truth ").centered());

        let example_ground_truth = example
            .example
            .ground_truth
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("n/a");

        let ground_truth = Paragraph::new(example_ground_truth)
            .wrap(Wrap { trim: true })
            .centered()
            .block(ground_truth_block);

        let label_block = Block::bordered()
            .padding(Padding { left: 1, right: 1, top: 1, bottom: 1 })
            .title(Line::from(" Label ").centered());

        let label_text = match example.label.as_ref() {
            Some(label) => {
                label.as_str().fg(Color::LightCyan).bg(Color::DarkGray).bold()
            }
            None => {
                "<None>".fg(Color::Black).bg(Color::LightYellow).bold()
            }
        };

        let label = Paragraph::new(label_text)
            .centered()
            .block(label_block);

        let instruction_lines = vec![
            Line::from(vec![
                Span::from(" ← ".fg(Color::White).bg(Color::DarkGray).bold()),
                Span::from(" Previous example")
            ]),
            Line::from(vec![
                Span::from(" → ".fg(Color::White).bg(Color::DarkGray).bold()),
                Span::from(" Next example")
            ]),
            Line::from(vec![
                Span::from(" ↑ ".fg(Color::White).bg(Color::DarkGray).bold()),
                Span::from(" Next label")
            ]),
            Line::from(vec![
                Span::from(" TAB ".fg(Color::White).bg(Color::DarkGray).bold()),
                Span::from(" Skip to first unlabelled")
            ]),
            Line::from(vec![
                Span::from(" Ctrl-C ".fg(Color::White).bg(Color::DarkGray).bold()),
                Span::from(" Save and exit")
            ])
        ];

        let instructions = Paragraph::new(instruction_lines);

        frame.render_widget(progress_gauge, progress);
        frame.render_widget(text, top_left);
        frame.render_widget(ground_truth, bottom_left);
        frame.render_widget(label, top_right);
        frame.render_widget(instructions, bottom_right);
    }
}
