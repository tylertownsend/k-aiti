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
    widgets::{Block, Borders, ListItem, List, Paragraph, ListState},
    Terminal,
};

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn new(items: Vec<T>) -> StatefulList<T> {
        let mut list = StatefulList {
            state:  ListState::default(),
            items,
        };

        list.state.select(Some(0));
        list
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}


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

    let choices = vec!["Yes", "No"];
    let mut choices_list = StatefulList::new(choices.into_iter().map(ListItem::new).collect::<Vec<_>>());
    loop {
        draw_has_account_screen(&mut terminal, &mut choices_list)?;
        match read()? {
            Event::Key(event) => match event.code {
                KeyCode::Up => {
                    choices_list.next();
                }
                KeyCode::Down => {
                    choices_list.previous();
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

fn draw_has_account_screen(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    choices_list: &mut StatefulList<ListItem>,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default().borders(Borders::ALL).title("OpenAI CLI - Profile Setup");
        f.render_widget(block, size);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Percentage(100),
                ]
                .as_ref(),
            )
            .split(size);

        let question = Paragraph::new("Do you have an OpenAI account?")
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center)
            .block(Block::default().title(Span::styled("Profile Setup", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))));
        f.render_widget(question, chunks[0]);

        let choices_widget = List::new(choices_list.items.clone())
            .block(Block::default().title("Select").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
            .highlight_symbol("> ");

        let h_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(33), Constraint::Percentage(34), Constraint::Percentage(33)].as_ref())
            .split(chunks[1]);

        f.render_stateful_widget(choices_widget, h_chunks[1], &mut choices_list.state);
    })?;

    Ok(())
}
fn draw_create_enter_openai_acount_screen(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
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
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Percentage(100),
                ]
                .as_ref(),
            )
            .split(size);

        let api_key_prompt = Paragraph::new("Enter your OpenAI API key:")
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left);
        f.render_widget(api_key_prompt, chunks[0]);

        let api_key_input = Paragraph::new("aPiKeY12345")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Left);
        f.render_widget(api_key_input, chunks[1]);

        let continue_widget = Paragraph::new("[Continue]")
            .style(Style::default().fg(Color::LightGreen))
            .alignment(Alignment::Right);
        f.render_widget(continue_widget, chunks[2]);
    })?;

    Ok(())
}

fn draw_create_openai_account_screen(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
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
                    Constraint::Length(1),
                    Constraint::Percentage(100),
                ]
                .as_ref(),
            )
            .split(size);

        let create_account_prompt = Paragraph::new("Create an OpenAI account and obtain your API key.")
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center);
        f.render_widget(create_account_prompt, chunks[0]);

        let open_signup_page_widget = Paragraph::new("[Open Signup Page]")
            .style(Style::default().fg(Color::LightGreen))
            .alignment(Alignment::Center);
        f.render_widget(open_signup_page_widget, chunks[1]);
    })?;

    Ok(())
}