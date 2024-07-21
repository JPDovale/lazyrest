use std::{
    io::{stdout, Result},
    vec,
};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{
            self, DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEventKind, KeyModifiers,
            MouseEventKind,
        },
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(EnableMouseCapture)?;
    enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut focus: Option<[u16; 2]> = None;

    loop {
        if event::poll(std::time::Duration::from_millis(10))? {
            let e = event::read()?;

            match e {
                event::Event::Key(key) => {
                    if key.kind == KeyEventKind::Press
                        && key.code == KeyCode::Char('q')
                        && key.modifiers.contains(KeyModifiers::CONTROL)
                    {
                        break;
                    }
                }
                event::Event::Mouse(mouse) => {
                    if mouse.kind == MouseEventKind::Down(event::MouseButton::Left) {
                        focus = Some([mouse.column, mouse.row]);
                    }
                }
                _ => {}
            }
        }

        terminal.draw(|frame| {
            let area = frame.size();

            let body = Layout::new(
                Direction::Vertical,
                vec![Constraint::Percentage(6), Constraint::Percentage(94)],
            )
            .split(area);

            let top = Layout::new(
                Direction::Horizontal,
                vec![Constraint::Percentage(90), Constraint::Percentage(10)],
            )
            .split(body[0]);

            let block_style = |rect: Rect| match focus {
                Some(mouse_in) => {
                    if mouse_in[0] >= rect.x
                        && mouse_in[0] <= rect.x + rect.width
                        && mouse_in[1] >= rect.y
                        && mouse_in[1] <= rect.y + rect.height
                    {
                        Style::default().fg(Color::Yellow).bg(Color::Black).bold()
                    } else {
                        Style::default().fg(Color::White).bg(Color::Black)
                    }
                }
                None => Style::default().fg(Color::White).bg(Color::Black),
            };

            frame.render_widget(
                Paragraph::new(format!("Clicked in: {:?}", focus)).block(
                    Block::new()
                        .borders(Borders::all())
                        .title("TermRest")
                        .style(block_style(top[0])),
                ),
                top[0],
            )
        })?;
    }

    stdout().execute(LeaveAlternateScreen)?;
    stdout().execute(DisableMouseCapture)?;
    disable_raw_mode()?;
    Ok(())
}
