use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    execute,
};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, ListItem, List, Paragraph, ListState, Wrap},
    Terminal,
};
use webbrowser;

use std::{io::stdout, env};

use super::config::{CreatedConfig, CreatedAccount};

#[derive(PartialEq, Eq, Hash, Clone)]
enum Screen {
    Intro,
    HasAccount,
    AccountSetup,
    NavigateToOpenAI,
    ProfileConfirmationPage,
    Disclaimer,
    CLIComplete,
    DetectedAccount,
    Done
}


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


pub fn run() -> Result<CreatedConfig, Box<dyn std::error::Error>> {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All))?;

    enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;


    let mut api_key_input = match env::var("OPENAI_API_KEY") {
        Ok(key) => key,
        Err(_) => String::new()
    };

    let mut previous_screen = Screen::Done;
    let mut current_screen = Screen::Intro;
    loop {
        match current_screen {
            Screen::Intro => {
                // Render intro screen
                loop {
                    draw_intro(&mut terminal)?;
                    match read()? {
                        Event::Key(event) => match event.kind {
                            KeyEventKind::Press => match event.code {
                                KeyCode::Enter => {
                                    previous_screen = current_screen;
                                    current_screen = if api_key_input != "" {
                                        Screen::DetectedAccount
                                    } else {
                                        Screen::HasAccount
                                    };
                                    break;
                                }
                                _ => {}
                            }
                            _ => {}
                        }
                        _ => {}
                    }
                }
                terminal.clear()?
            }
            Screen::DetectedAccount => {
                let choices = vec!["Yes", "No"];
                let mut choices_list = StatefulList::new(choices.clone().into_iter().map(ListItem::new).collect::<Vec<_>>());
                loop {
                    draw_account_found(&mut terminal, &mut choices_list)?;
                    match read()? {
                        Event::Key(event) => match event.kind {
                            KeyEventKind::Press => match event.code {
                                KeyCode::Up => {
                                    choices_list.next();
                                }
                                KeyCode::Down => {
                                    choices_list.previous();
                                }
                                KeyCode::Enter => {
                                    // Proceed to the next screen based on the user's selection
                                    if let Some(selected_index) = &choices_list.state.selected() {
                                        if choices[*selected_index] == "Yes" {
                                            previous_screen = current_screen;
                                            current_screen = Screen::AccountSetup;
                                        } else {
                                            previous_screen = current_screen;
                                            current_screen = Screen::HasAccount;
                                        }
                                    }
                                    break;
                                }
                                KeyCode::Char('b') | KeyCode::Char('B') => {
                                    let temp = previous_screen;
                                    previous_screen = current_screen;
                                    current_screen = temp;
                                    break;
                                }
                                _ => {}
                            },
                            _ => {}
                        }
                        _ => {}
                    }
                }
                terminal.clear()?;
            },
            Screen::HasAccount => {
                // Render has_account screen and handle user input
                // Update current_screen based on user input
                let choices = vec!["Yes", "No"];
                let mut choices_list = StatefulList::new(choices.clone().into_iter().map(ListItem::new).collect::<Vec<_>>());
                loop {
                    draw_has_account_screen(&mut terminal, &mut choices_list)?;
                    match read()? {
                        Event::Key(event) => match event.kind {
                            KeyEventKind::Press => match event.code {
                                KeyCode::Up => {
                                    choices_list.next();
                                }
                                KeyCode::Down => {
                                    choices_list.previous();
                                }
                                KeyCode::Enter => {
                                    // Proceed to the next screen based on the user's selection
                                    if let Some(selected_index) = &choices_list.state.selected() {
                                        if choices[*selected_index] == "Yes" {
                                            previous_screen = current_screen;
                                            current_screen = Screen::AccountSetup;
                                        } else {
                                            previous_screen = current_screen;
                                            current_screen = Screen::NavigateToOpenAI;
                                        }
                                    }
                                    break;
                                }
                                KeyCode::Char('b') | KeyCode::Char('B') => {
                                    let temp = previous_screen;
                                    previous_screen = current_screen;
                                    current_screen = temp;
                                    break;
                                }
                                _ => {}
                            },
                            _ => {}
                        }
                        _ => {}
                    }
                }
                terminal.clear()?
            }, 
            Screen::NavigateToOpenAI => {
                // Render no_condition screen and handle user input
                // Update current_screen based on user input
                create_account();
                loop {
                    draw_create_openai_account_screen(&mut terminal)?;
                    match read()? {
                        Event::Key(event) => match event.kind {
                            KeyEventKind::Press => match event.code {
                                KeyCode::Char('c') | KeyCode::Char('C') => {
                                    previous_screen = current_screen;
                                    current_screen = Screen::AccountSetup;
                                    break;
                                },
                                KeyCode::Char('b') | KeyCode::Char('B') => {
                                    let temp = previous_screen;
                                    previous_screen = current_screen;
                                    current_screen = temp;
                                    break;
                                },
                                _ => {}
                            },
                            _ => {}
                        }
                        _ => {}
                    }
                }
                terminal.clear()?;
            },
            Screen::AccountSetup => {
                // Render yes_condition screen and handle user input
                // Update current_screen based on user input
                // Render intro screen
                let mut editing_field: bool = false;
                loop {
                    draw_enter_openai_acount_screen(&mut terminal, &mut api_key_input, &mut editing_field)?;
                    match read()? {
                        Event::Key(event) => match event.kind {
                            KeyEventKind::Press => match event.code {
                                KeyCode::Char('c') | KeyCode::Char('C') => {
                                    if !editing_field {
                                        // Proceed to the next screen
                                        previous_screen = current_screen;
                                        current_screen = Screen::ProfileConfirmationPage;
                                        break;
                                    } else {
                                        editing_field = true;
                                    }
                                }
                                KeyCode::Char('b') | KeyCode::Char('B') => {
                                    if !editing_field {
                                        let temp = previous_screen;
                                        previous_screen = current_screen;
                                        current_screen = temp;
                                        break;
                                    }
                                },
                                KeyCode::Backspace => {
                                    if editing_field {
                                        api_key_input.pop();
                                    }
                                },
                                KeyCode::Char(c) => {
                                    if editing_field {
                                        api_key_input.push(c);
                                    }
                                },
                                KeyCode::Enter => {
                                    if editing_field {
                                        editing_field = false;
                                    } else {
                                        editing_field = true;
                                    }
                                },
                                _ => {}
                            },
                            _ => {}
                        }
                        _ => {}
                        
                    }
                }
                terminal.clear()?;
            } 
            Screen::ProfileConfirmationPage => {
                let choices = vec!["Yes", "No"];
                let mut choices_list = StatefulList::new(choices.clone().into_iter().map(ListItem::new).collect::<Vec<_>>());
                loop {
                    draw_profile_confirmation_screen(&mut terminal, api_key_input.as_str(), &mut choices_list)?;
                    match read()? {
                        Event::Key(event) => match event.kind {
                            KeyEventKind::Press => match event.code {
                                KeyCode::Up => {
                                    choices_list.next();
                                }
                                KeyCode::Down => {
                                    choices_list.previous();
                                }
                                KeyCode::Enter => {
                                    // Proceed to the next screen based on the user's selection
                                    if let Some(selected_index) = &choices_list.state.selected() {
                                        if choices[*selected_index] == "Yes" {
                                            previous_screen = current_screen;
                                            current_screen = Screen::Disclaimer;
                                        } else {
                                            let temp = previous_screen;
                                            previous_screen = current_screen;
                                            current_screen = temp;
                                            break;
                                        }
                                    }
                                    break;
                                }
                                KeyCode::Char('b') | KeyCode::Char('B') => {
                                    let temp = previous_screen;
                                    previous_screen = current_screen;
                                    current_screen = temp;
                                    break;
                                }
                                _ => {}
                            },
                            _ => {}
                        }
                        _ => {}
                    }
                }
                terminal.clear()?;
            },
            Screen::Disclaimer => {
                loop {
                    draw_disclaimer_screen(&mut terminal)?; 
                    match read()? {
                        Event::Key(event) => match event.kind {
                            KeyEventKind::Press => match event.code {
                                KeyCode::Enter => {
                                    previous_screen = current_screen;
                                    current_screen = Screen::CLIComplete;
                                    break;
                                },
                                KeyCode::Char('b') | KeyCode::Char('B') => {
                                    let temp = previous_screen;
                                    previous_screen = current_screen;
                                    current_screen = temp;
                                    break;
                                },
                                _ => {}
                            },
                            _ => {}
                        },
                        _ => {}
                    }
                }
                terminal.clear()?;
            },
            Screen::CLIComplete => {
                loop {
                    draw_profile_setup_complete_screen(&mut terminal)?; 
                    match read()? {
                        Event::Key(event) => match event.kind {
                            KeyEventKind::Press => match event.code {
                                KeyCode::Enter => {
                                    previous_screen = current_screen;
                                    current_screen = Screen::Done;
                                    break;
                                }
                                _ => {}
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
                terminal.clear()?;
            },
            Screen::Done => {
                break;
            }
        }
    }
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), Clear(ClearType::All))?;

    let created_profile = CreatedConfig {
        user_name: "".to_string(),
        accounts: vec![
            CreatedAccount {
                name: "OpenAI".to_string(),
                env_var_name: "OPENAI_API_KEY".to_string(),
                env_var_value: api_key_input,
                create_env_var: true
            }
        ]
    };
    Ok(created_profile)
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

fn draw_account_found(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    choices_list: &mut StatefulList<ListItem>,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default().borders(Borders::ALL).title("OpenAI CLI - Account Detection");
        f.render_widget(block, size);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(size);

        let detected_prompt = Paragraph::new("We have detected environment variable OPENAI_API_KEY in your system.")
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left);
        f.render_widget(detected_prompt, chunks[0]);

        let use_api_key_prompt = Paragraph::new("Would you like to use this as part of your openai account?")
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left);
        f.render_widget(use_api_key_prompt, chunks[1]);

        let choices_widget = List::new(choices_list.items.clone())
            .block(Block::default().borders(Borders::NONE))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().bg(Color::DarkGray).fg(Color::White))
            .highlight_symbol("> ");
        f.render_stateful_widget(choices_widget, chunks[2], &mut choices_list.state);

        let action_text = "[Enter] Select [B] Back";
        let action_span = Span::styled(action_text, Style::default().fg(Color::LightGreen));
        let action_block = Block::default().title(action_span).borders(Borders::ALL);
        f.render_widget(action_block, chunks[3]);
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
                    Constraint::Length(3),
                    Constraint::Length(3)
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

        let action_text = "[Enter] Select [B] Back";
        let action_span = Span::styled(action_text, Style::default().fg(Color::LightGreen));
        let action_block = Block::default().title(action_span).borders(Borders::ALL);
        f.render_widget(action_block, chunks[2]);
    })?;

    Ok(())
}
fn draw_enter_openai_acount_screen(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    api_key: &str,
    editing_field: &mut bool,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal.draw(|f| {
        let size = f.size();
        let title = "OpenAI CLI - Profile Setup";
            let title_span = Span::styled(
                title,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            );
            let title_block = Block::default().title(title_span).borders(Borders::ALL);
            f.render_widget(title_block, size);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(3),
                    Constraint::Length(3)
                ]
                .as_ref(),
            )
            .split(size);

        let api_key_prompt = Paragraph::new("Enter your OpenAI API key:")
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left);
        f.render_widget(api_key_prompt, chunks[0]);

        let api_key_style = if *editing_field {
            Style::default().fg(Color::Yellow).bg(Color::DarkGray)
        } else {
            Style::default().fg(Color::Yellow)
        };

        let api_key_text = if *editing_field {
            format!("{}|", api_key)
        } else {
            api_key.to_string()
        };

        let input_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White));

        let api_key_input = Paragraph::new(api_key_text)
            .style(api_key_style)
            .block(input_block)
            .alignment(Alignment::Left);
        f.render_widget(api_key_input, chunks[1]);

        let action_text = "[B] Back";
        let action_span = Span::styled(action_text, Style::default().fg(Color::LightGreen));
        let action_block = Block::default().title(action_span).borders(Borders::ALL);
        f.render_widget(action_block, chunks[2]);
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
                    Constraint::Length(3),
                    Constraint::Length(3)
                ]
                .as_ref(),
            )
            .split(size);

        let create_account_prompt = Paragraph::new("Create an OpenAI account and obtain your API key.")
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center);
        f.render_widget(create_account_prompt, chunks[0]);

        let open_signup_page_widget = Paragraph::new("If your browser hasn't opened, please use the following link: \nhttps://platform.openai.com/account/api-keys")
            .style(Style::default().fg(Color::LightGreen))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        f.render_widget(open_signup_page_widget, chunks[1]);

        let action_text = "[Enter] Continue [B] Back";
        let action_span = Span::styled(action_text, Style::default().fg(Color::LightGreen));
        let action_block = Block::default().title(action_span).borders(Borders::ALL);
        f.render_widget(action_block, chunks[2]);
    })?;

    Ok(())
}

fn create_account() {
    let res = webbrowser::open("https://platform.openai.com/account/api-keys").is_err();
    if res {
        // encountered error
    }
}

fn draw_profile_confirmation_screen(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    api_key: &str,
    choices_list: &mut StatefulList<ListItem>,
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
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3)
                ]
                .as_ref(),
            )
            .split(size);

        let review_prompt = Paragraph::new("Review your profile information:")
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left);
        f.render_widget(review_prompt, chunks[0]);

        let api_key_display = format!("API Key: {}", api_key);
        let api_key_paragraph = Paragraph::new(api_key_display)
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Left);
        f.render_widget(api_key_paragraph, chunks[1]);

        let looks_good_prompt = Paragraph::new("Looks good?")
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left);
        f.render_widget(looks_good_prompt, chunks[2]);

        let choices_widget = List::new(choices_list.items.clone())
            .block(Block::default().borders(Borders::NONE))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().bg(Color::DarkGray).fg(Color::White))
            .highlight_symbol("> ");
        f.render_stateful_widget(choices_widget, chunks[3], &mut choices_list.state);

        let action_text = "[Enter] Continue [B] Back";
        let action_span = Span::styled(action_text, Style::default().fg(Color::LightGreen));
        let action_block = Block::default().title(action_span).borders(Borders::ALL);
        f.render_widget(action_block, chunks[4]);
    })?;

    Ok(())
}

fn draw_disclaimer_screen(
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
                    Constraint::Length(5),
                    Constraint::Percentage(100),
                ]
                .as_ref(),
            )
            .split(size);

        let disclaimer_text = "Disclaimer: Always keep your API key secure \
                               and don't share it with others. Unauthorized \
                               usage may result in billing charges and other \
                               consequences.";

        let disclaimer_paragraph = Paragraph::new(disclaimer_text)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });
        f.render_widget(disclaimer_paragraph, chunks[0]);

        let action_text = "[Enter] Continue [B] Back";
        let action_span = Span::styled(action_text, Style::default().fg(Color::LightGreen));
        let action_block = Block::default().title(action_span).borders(Borders::ALL);
        f.render_widget(action_block, chunks[1]);
    })?;

    Ok(())
}

fn draw_profile_setup_complete_screen(
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
                    Constraint::Length(3),
                    Constraint::Percentage(100),
                ]
                .as_ref(),
            )
            .split(size);

        let completion_text = "Profile setup is now complete. Enjoy using the OpenAI CLI!";

        let completion_paragraph = Paragraph::new(completion_text)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });
        f.render_widget(completion_paragraph, chunks[0]);

        let action_text = "[Enter] Finish";
        let action_span = Span::styled(action_text, Style::default().fg(Color::LightGreen));
        let action_block = Block::default().title(action_span).borders(Borders::ALL);
        f.render_widget(action_block, chunks[1]);
    })?;

    Ok(())
}