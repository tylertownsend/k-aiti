use crossterm::{
    event::{read, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    execute,
};
use tui::{
    backend::CrosstermBackend,
    widgets::{ListItem, Block, Borders, List},
    Terminal, layout::Rect, style::{Style, Modifier},
};

use std::io::{stdout, Stdout};

mod api_account;
mod profile;
mod stateful_list;

use super::{
    config::{ CreatedConfig, CreatedAccount},
    environment_variables::EnvironmentVariableHandler,
};

use self::{
    profile::{draw_intro, draw_profile_setup_complete_screen, draw_profile_confirmation_screen}, 
    api_account::{draw_account_found, draw_has_account_screen, draw_create_openai_account_screen, create_account, draw_enter_openai_account_screen, draw_disclaimer_screen},
    stateful_list::StatefulList
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
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

struct PopupResult {
    abort: bool
}


struct ProfileSetupState {
    previous_screen: Screen,
    current_screen: Screen,
    retrieve_key: bool,
    api_key_input: String,
    terminal: Terminal<CrosstermBackend<Stdout>>,
    /**
     * Abort the setup process
     */
    abort: bool
}

impl ProfileSetupState {
    pub fn abort_view(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let result = draw_popup(&mut self.terminal)?;
        if result.abort {
            self.abort = true;
            self.current_screen = Screen::Done;
        }
        Ok(())
    }
}

pub fn run(env_var_handler: &Box<dyn EnvironmentVariableHandler>) -> Result<CreatedConfig, Box<dyn std::error::Error>> {
    let api_key_input = match env_var_handler.get("OPENAI_API_KEY") {
        Ok(key) => key,
        Err(_) => String::new()
    };

    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All))?;
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;


    let retrieve_key = false;

    let previous_screen = Screen::Done;
    let current_screen = Screen::Intro;
    let mut state = ProfileSetupState { 
        previous_screen,
        current_screen, 
        retrieve_key,
        api_key_input, 
        terminal,
        abort: false,
    };

    loop {
        state.terminal.clear()?;
        match state.current_screen {
            Screen::Intro => {
                render_intro_screen(&mut state)?;
                state.terminal.clear()?
            }
            Screen::DetectedAccount => {
                account_detected_view(&mut state)?;
                state.terminal.clear()?;
            },
            Screen::HasAccount => {
                account_lookup_view(&mut state)?;
                state.terminal.clear()?
            }, 
            Screen::NavigateToOpenAI => {
                account_creation_view(&mut state)?;
                state.terminal.clear()?;
            },
            Screen::AccountSetup => {
                profile_setup_view(&mut state)?;
                state.terminal.clear()?;
            } 
            Screen::ProfileConfirmationPage => {
                profile_confirmation_view(&mut state)?;
                state.terminal.clear()?;
            },
            Screen::Disclaimer => {
                profile_disclaimer_view(&mut state)?; 
                state.terminal.clear()?;
            },
            Screen::CLIComplete => {
                cli_setup_complete_view(&mut state)?;
                state.terminal.clear()?;
            },
            Screen::Done => break
        }
    }
    disable_raw_mode()?;
    execute!(state.terminal.backend_mut(), Clear(ClearType::All))?;
    state.terminal.clear()?;
    let created_profile = CreatedConfig {
        user_name: "".to_string(),
        accounts: vec![
            CreatedAccount {
                name: "OpenAI".to_string(),
                env_var_name: "OPENAI_API_KEY".to_string(),
                env_var_value: state.api_key_input,
                create_env_var: true
            }
        ],
        abort: state.abort
    };
    Ok(created_profile)
}

fn draw_popup(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>
) -> Result<PopupResult, Box<dyn std::error::Error>> {
    let mut abort = false;

    let choices = vec!["Yes", "No"];
    let mut choices_list = StatefulList::new(choices.clone().into_iter().map(ListItem::new).collect::<Vec<_>>());
    loop {
        terminal.draw(|f| {
            let size = f.size();

            // Center the popup
            let popup = Rect {
                x: size.width / 4,
                y: size.height / 4,
                width: size.width / 2,
                height: size.height / 2,
            };

            let block = Block::default()
                .title("Are you sure you want to abandon?")
                .borders(Borders::ALL);
            f.render_widget(block, popup);

            let choices_rect = Rect {
                x: popup.x + 2,
                y: popup.y + 1,
                width: popup.width - 4,
                height: popup.height - 2,
            };

            let list = List::new(choices_list.items.clone())
                .block(Block::default())
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(">");
            
            f.render_stateful_widget(list, choices_rect, &mut choices_list.state);
        })?;
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
                                abort = true;
                            }
                        }
                        break;
                    }
                    KeyCode::Esc => {
                        break;
                    }
                    _ => {}
                }
                _ => {}
            }
            _ => {}
        }
    }

    Ok(PopupResult { abort })
}

fn render_intro_screen(state: &mut ProfileSetupState) -> Result<(), Box<dyn std::error::Error>> {
    // Render intro screen

    loop {
        draw_intro(&mut state.terminal)?;
        match read()? {
            Event::Key(event) => match event.kind {
                KeyEventKind::Press => match event.code {
                    KeyCode::Enter => {
                        state.previous_screen = state.current_screen;
                        state.current_screen = if state.api_key_input != "" {
                            Screen::DetectedAccount
                        } else {
                            Screen::HasAccount
                        };
                        break;
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        state.abort_view()?;
                        if state.abort {
                            break;
                        }
                    },
                    _ => {}
                }
                _ => {}
            }
            _ => {}
        }
    }
    Ok(())
} 

fn account_detected_view(state: &mut ProfileSetupState) -> Result<(), Box<dyn std::error::Error>> {
    let choices = vec!["Yes", "No"];
    let mut choices_list = StatefulList::new(choices.clone().into_iter().map(ListItem::new).collect::<Vec<_>>());
    loop {
        draw_account_found(&mut state.terminal, &mut choices_list)?;
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
                                state.previous_screen = state.current_screen;
                                state.current_screen = Screen::AccountSetup;
                            } else {
                                state.previous_screen = state.current_screen;
                                state.current_screen = Screen::HasAccount;
                            }
                        }
                        break;
                    }
                    KeyCode::Char('b') | KeyCode::Char('B') => {
                        let temp = state.previous_screen;
                        state.previous_screen = state.current_screen;
                        state.current_screen = temp;
                        break;
                    }
                    _ => {}
                },
                _ => {}
            }
            _ => {}
        }
    }
    Ok(())
}

fn account_lookup_view(state: &mut ProfileSetupState) -> Result<(), Box<dyn std::error::Error>> {
    // Render has_account screen and handle user input
    // Update current_screen based on user input
    state.retrieve_key = false;
    let choices = vec!["Yes", "No"];
    let mut choices_list = StatefulList::new(choices.clone().into_iter().map(ListItem::new).collect::<Vec<_>>());
    loop {
        draw_has_account_screen(&mut state.terminal, &mut choices_list)?;
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
                                state.previous_screen = state.current_screen;
                                state.retrieve_key = true;
                                state.current_screen = Screen::AccountSetup;
                            } else {
                                state.previous_screen = state.current_screen;
                                state.current_screen = Screen::NavigateToOpenAI;
                            }
                        }
                        break;
                    }
                    KeyCode::Char('b') | KeyCode::Char('B') => {
                        let temp = state.previous_screen;
                        state.previous_screen = state.current_screen;
                        state.current_screen = temp;
                        break;
                    }
                    _ => {}
                },
                _ => {}
            }
            _ => {}
        }
    }
    Ok(())
}

fn account_creation_view(state: &mut ProfileSetupState) -> Result<(), Box<dyn std::error::Error>> {
    // Render no_condition screen and handle user input
    // Update current_screen based on user input
    create_account();
    loop {
        draw_create_openai_account_screen(&mut state.terminal)?;
        match read()? {
            Event::Key(event) => match event.kind {
                KeyEventKind::Press => match event.code {
                    KeyCode::Enter => {
                        state.previous_screen = state.current_screen;
                        state.current_screen = Screen::AccountSetup;
                        break;
                    },
                    KeyCode::Char('b') | KeyCode::Char('B') => {
                        let temp = state.previous_screen;
                        state.previous_screen = state.current_screen;
                        state.current_screen = temp;
                        break;
                    },
                    _ => {}
                },
                _ => {}
            }
            _ => {}
        }
    }
    Ok(())
}

fn profile_setup_view(state: &mut ProfileSetupState) -> Result<(), Box<dyn std::error::Error>> {
    // Render yes_condition screen and handle user input
    // Update current_screen based on user input
    // Render intro screen
    if state.retrieve_key {
        create_account();
    }
    let mut editing_field: bool = false;
    loop {
        draw_enter_openai_account_screen(&mut state.terminal, &mut state.api_key_input, &mut editing_field)?;
        match read()? {
            Event::Key(event) => match event.kind {
                KeyEventKind::Press => match event.code {
                    KeyCode::Char('e') | KeyCode::Char('E') => {
                        if !editing_field {
                            editing_field = true;
                        }
                    }
                    KeyCode::Char('b') | KeyCode::Char('B') => {
                        if !editing_field {
                            let temp = state.previous_screen;
                            state.previous_screen = state.current_screen;
                            state.current_screen = temp;
                            break;
                        }
                    },
                    KeyCode::Backspace => {
                        if editing_field {
                            state.api_key_input.pop();
                        }
                    },
                    KeyCode::Char(c) => {
                        if editing_field {
                            state.api_key_input.push(c);
                        }
                    },
                    KeyCode::Enter => {
                        if editing_field {
                            continue;
                        } 
                        // Proceed to the next screen
                        state.previous_screen = state.current_screen;
                        state.current_screen = Screen::ProfileConfirmationPage;
                        break;
                        
                    },
                    KeyCode::Esc => {
                        if editing_field {
                            editing_field = false;
                        }
                    },
                    _ => {}
                },
                _ => {}
            }
            _ => {}
            
        }
    }
    Ok(())
}

fn profile_confirmation_view(state: &mut ProfileSetupState) -> Result<(), Box<dyn std::error::Error>> {
    let choices = vec!["Yes", "No"];
    let mut choices_list = StatefulList::new(choices.clone().into_iter().map(ListItem::new).collect::<Vec<_>>());
    loop {
        draw_profile_confirmation_screen(&mut state.terminal, &mut state.api_key_input.as_str(), &mut choices_list)?;
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
                                state.previous_screen = state.current_screen;
                                state.current_screen = Screen::Disclaimer;
                            } else {
                                let temp = state.previous_screen;
                                state.previous_screen = state.current_screen;
                                state.current_screen = temp;
                                break;
                            }
                        }
                        break;
                    }
                    KeyCode::Char('b') | KeyCode::Char('B') => {
                        let temp = state.previous_screen;
                        state.previous_screen = state.current_screen;
                        state.current_screen = temp;
                        break;
                    }
                    _ => {}
                },
                _ => {}
            }
            _ => {}
        }
    }
    Ok(())
}

fn profile_disclaimer_view(state: &mut ProfileSetupState) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        draw_disclaimer_screen(&mut state.terminal)?; 
        match read()? {
            Event::Key(event) => match event.kind {
                KeyEventKind::Press => match event.code {
                    KeyCode::Enter => {
                        state.previous_screen = state.current_screen;
                        state.current_screen = Screen::CLIComplete;
                        break;
                    },
                    KeyCode::Char('b') | KeyCode::Char('B') => {
                        let temp = state.previous_screen;
                        state.previous_screen = state.current_screen;
                        state.current_screen = temp;
                        break;
                    },
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
    }
    Ok(())
}

fn cli_setup_complete_view(state: &mut ProfileSetupState) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        draw_profile_setup_complete_screen(&mut state.terminal)?; 
        match read()? {
            Event::Key(event) => match event.kind {
                KeyEventKind::Press => match event.code {
                    KeyCode::Enter => {
                        state.previous_screen = state.current_screen;
                        state.current_screen = Screen::Done;
                        break;
                    }
                    _ => {}
                }
                _ => {}
            },
            _ => {}
        }
    }
    Ok(())
}