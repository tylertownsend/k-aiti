use crossterm::{
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    execute,
};
use std::{
    io::{stdout, Write},
    time::Duration,
};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, ListItem, List, Paragraph},
    Terminal,
};

pub fn run() -> Result<bool, Box<dyn std::error::Error>> {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All))?;

    enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    loop {
        draw_intro(&mut terminal)?;
        if let Event::Key(event) = read()? {
            if event.code == KeyCode::Enter {
                break;
            }
        }
    }

    let mut selection = 0;
    loop {
        draw_has_account_screen(&mut terminal, &mut selection)?;
        match read()? {
            Event::Key(event) => match event.code {
                KeyCode::Up => {
                    if selection > 0 {
                        selection -= 1;
                    }
                }
                KeyCode::Down => {
                    if selection < 1 {
                        selection += 1;
                    }
                }
                KeyCode::Enter => {
                    // Proceed to the next screen based on the user's selection
                    break;
                }
                _ => {}
            },
            _ => {}
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), Clear(ClearType::All))?;

    Ok(true)
}

// intro
pub fn draw_intro(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<(), Box<dyn std::error::Error>> {
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default().borders(Borders::ALL).title("OpenAI CLI - Profile Setup");
        f.render_widget(block, size);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(1),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(size);

        let intro_text = "Welcome to the OpenAI CLI setup!\nFollow the instructions to configure your user profile.";
        let intro = Paragraph::new(intro_text)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center)
            .block(Block::default().title(Span::styled("Introduction", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))));
        f.render_widget(intro, chunks[1]);
    })?;

    Ok(())
}

// setup
fn draw_has_account_screen(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    selection: &mut usize,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default().borders(Borders::ALL).title("OpenAI CLI - Profile Setup");
        f.render_widget(block, size);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Percentage(50),
                ]
                .as_ref(),
            )
            .split(size);

        let question = Paragraph::new("Do you have an OpenAI account?")
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center)
            .block(Block::default().title(Span::styled("Profile Setup", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))));
        f.render_widget(question, chunks[1]);

        let choices = vec!["Yes", "No"];
        let choices_widget = menu_list_widget(&choices, selection.clone());
        let menu_area = centered_rect(20, choices.len() as u16 + 2, size);
        f.render_widget(choices_widget, menu_area);
    })?;

    Ok(())
}

fn centered_rect(width: u16, height: u16, parent: Rect) -> Rect {
    let x = (parent.width.saturating_sub(width)) / 2;
    let y = (parent.height.saturating_sub(height)) / 2;
    Rect::new(x, y, width, height)
}

fn menu_list_widget<'a>(
    items: &'a [&'a str],
    selected_item: usize,
) -> List<'a> {
    let menu_items: Vec<_> = items
        .iter()
        .enumerate()
        .map(|(index, item)| {
            let label = *item;
            let style = if index == selected_item {
                Style::default().fg(Color::LightGreen)
            } else {
                Style::default()
            };
            ListItem::new(Span::styled(label, style))
        })
        .collect();

    List::new(menu_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Choose an option"),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
}