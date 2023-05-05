use crossterm::event::{self, Event, KeyCode};
use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use crate::user_profile::config::{Account, Config};

pub fn present_profile() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config {
        user_name: String::from("John Doe"),
        accounts: vec![
            Account {
                name: String::from("Personal"),
                env_var_name: String::from("PERSONAL_VAR"),
            },
            Account {
                name: String::from("Work"),
                env_var_name: String::from("WORK_VAR"),
            },
        ],
    };

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                .split(f.size());

            // Left panel
            let items = vec![ListItem::new("Accounts"), ListItem::new("Settings")];
            let list = List::new(items).block(Block::default().borders(Borders::ALL).title("Menu"));
            f.render_widget(list, chunks[0]);

            // Right panel
            let title = Spans::from(vec![Span::styled(
                format!("User Name: {}", config.user_name),
                Style::default().fg(Color::LightGreen),
            )]);

            let account_info = config
                .accounts
                .iter()
                .map(|account| {
                    format!("{} ({})", account.name, account.env_var_name)
                })
                .collect::<Vec<String>>()
                .join("\n");

            let accounts = Spans::from(vec![Span::styled(
                format!("Accounts:\n{}", account_info),
                Style::default().fg(Color::Yellow),
            )]);

            let settings = Spans::from(vec![Span::styled(
                "Settings:\n- Theme: Dark\n- Auto Save: Enabled\n- Notifications: On",
                Style::default().fg(Color::White),
            )]);

            let content = Paragraph::new(vec![title, accounts, settings])
                .block(Block::default().borders(Borders::ALL).title("Profile Editor"))
                .style(Style::default().fg(Color::White));

            f.render_widget(content, chunks[1]);
        })?;

        if let Event::Key(event) = event::read()? {
            if event.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    terminal.clear()?;
    Ok(())
}