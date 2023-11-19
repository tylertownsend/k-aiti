use std::io;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};

use crate::config::user::settings::SettingsConfig as Config;
use super::view_model;
use super::super::ui::StatefulList;


pub fn draw_view_models(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, config: &mut Config) -> Result<(), Box<dyn std::error::Error>> {
    let exit = false;

    let active_panel = 0;

    // Initialize model list
    // let models = vec!["Model 1", "Model 2", "Model 3", "Model 4"];
    let model_names = config
        .models
        .iter()
        .map(|model| ListItem::new(model.name.clone()))
        .collect::<Vec<ListItem>>();

    // let model_ids = config
    //     .models
    //     .iter()
    //     .map(|model| model.id.clone())
    //     .collect::<Vec<String>>();

    let mut models_list = StatefulList::new(model_names);

    let actions = vec![
        "[V] View/Edit",
        // "[A] Add",
        // "[R] Remove",
        "[B] Back",
    ];
    let mut actions_list = StatefulList::new(actions.into_iter().map(ListItem::new).collect::<Vec<_>>());
    while !exit {
        terminal.draw(|frame| {
            let size = frame.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3), // ViewModels title
                        Constraint::Percentage(100),
                    ]
                    .as_ref(),
                )
                .split(size);

            let block = Block::default().title("View Models").borders(Borders::ALL);
            frame.render_widget(block, chunks[0]);

            let h_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .split(chunks[1]);

            let models_widget = List::new(models_list.items.clone())
                .block(Block::default().title("Models").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
                .highlight_symbol("> ");

            let actions_widget = List::new(actions_list.items.clone())
                .block(Block::default().title("Actions").borders(Borders::ALL))
                .style(Style::default().fg(Color::White));
                // .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
                // .highlight_symbol("> ");

            frame.render_stateful_widget(models_widget, h_chunks[1], &mut models_list.state);
            frame.render_stateful_widget(actions_widget, h_chunks[0], &mut actions_list.state);
        })?;

        match event::read()? {
            Event::Key(event) => match event.kind {
                KeyEventKind::Press => match event.code {
                    KeyCode::Left | KeyCode::Right => {
                        // active_panel = 1 - active_panel;
                    }
                    KeyCode::Up => {
                        if active_panel == 0 {
                            models_list.previous();
                        } else {
                            actions_list.previous();
                        }
                    }
                    KeyCode::Down => {
                        if active_panel == 0 {
                            models_list.next();
                        } else {
                            actions_list.next();
                        }
                    }
                    KeyCode::Char('v') | KeyCode::Char('V') => {
                        // View logic
                        if let Some(selected_index) = models_list.state.selected() {
                            if let Some(selected_model) = config.models.get_mut(selected_index) {
                                view_model::view(terminal, selected_model)?;
                            }
                        }
                    }
                    KeyCode::Enter => {
                        // Enter automatically view/edits the model
                        if let Some(selected_index) = models_list.state.selected() {
                            if let Some(selected_model) = config.models.get_mut(selected_index) {
                                view_model::view(terminal, selected_model)?;
                            }
                        }
                    }
                    KeyCode::Char('a') | KeyCode::Char('A') => {
                        // Add logic
                    }
                    KeyCode::Char('r') | KeyCode::Char('R') => {
                        // Remove logic
                    }
                    KeyCode::Char('e') | KeyCode::Char('E') => {
                        // Edit logic
                    }
                    KeyCode::Char('b') | KeyCode::Char('B') => {
                        // Go back to the previous screen
                        // disable_raw_mode()?;
                        break;
                    }
                    KeyCode::Esc => {
                        // disable_raw_mode()?;
                        break;
                    }
                    _ => {}
                },
                _ => {

                }
            },
            _ => {}
        }
    }
    terminal.clear()?;
    Ok(())
}