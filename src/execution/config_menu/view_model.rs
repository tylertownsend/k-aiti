use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::{io, error::Error};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Alignment, Corner},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, List, ListItem},
    Terminal,
};

use crate::config::user::settings::ModelConfig;
use super::stateful_list::StatefulList;

pub fn view(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, c_model: &mut ModelConfig) -> Result<(), Box<dyn Error>> {
    let required_config = ModelRequiredConfig{
        id: c_model.id.to_string(),
        name: c_model.name.to_string()
    };
    view_config(terminal, &required_config, &mut c_model.config)
}

fn get_config_value_as_string(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Array(arr) => arr
            .iter()
            .map(get_config_value_as_string)
            .collect::<Vec<String>>()
            .join(", "),
        _ => "Unknown value".to_string(),
    }
}

struct ModelRequiredConfig {
    id: String,
    name: String,
}

fn view_config(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    required_config: &ModelRequiredConfig,
    config: &mut serde_json::Value,
) -> Result<(), Box<dyn Error>> {

    // let required_fields = vec![String::from("ID"), String::from("Name")];
    // let required_widget = vec![String::from("ID"), String::from("Name")];

    let config_fields: Vec<String> = config
        .as_object()
        .unwrap()
        .keys()
        .map(|key| key.to_string())
        .collect();
    let config_widgets: Vec<String> = config_fields
        .iter()
        .map(|field| {
            get_config_value_as_string(&config[field])
        })
        .collect();

    let mut state = MenuState {
        config_fields,
        config_widgets,
        required_config,
        editing_field: false,
        scroll_offset: 0,
        selected_field: 0,
        unsaved_changes: false
    };

    loop {
        present_menu(terminal, &mut state)?;
        match event::read()? {
            Event::Key(event) => match event.kind {
                KeyEventKind::Press => match event.code {
                    KeyCode::Char(c) => {
                        if state.editing_field {
                            state.unsaved_changes = true;
                            state.config_widgets[state.selected_field].push(c);
                        } else {
                            // Handle special cases for certain characters
                            match c {
                                's' | 'S' => {
                                    if state.unsaved_changes {
                                        let confirmed = present_confirmation(terminal)?;
                                        if confirmed {
                                            // Save changes
                                            state.unsaved_changes = false;
                                            for (field, value) in state.config_fields.iter().zip(state.config_widgets.iter()) {
                                                config[field] = serde_json::Value::String(value.clone());
                                            }
                                        }
                                    }
                                }
                                'b' | 'B' => {
                                    if state.unsaved_changes {
                                        let abort_confirmed = present_abort_confirmation(terminal)?;
                                        if abort_confirmed {
                                            state.editing_field = false;
                                            break;
                                        }
                                    } else {
                                        state.editing_field = false;
                                        break;
                                    }
                                }
                                'e' | 'E' => {
                                    state.editing_field = !state.editing_field;
                                }
                                _ => {}
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        if state.editing_field {
                            state.unsaved_changes = true;
                            state.config_widgets[state.selected_field].pop();
                        }
                    }
                    KeyCode::Esc => {
                        if state.editing_field {
                            state.editing_field = false;
                        } else {
                            if state.unsaved_changes {
                                let abort_confirmed = present_abort_confirmation(terminal)?;
                                if abort_confirmed {
                                    // don't save changes
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }
                    KeyCode::Tab => {
                        if state.editing_field {
                            state.selected_field = (state.selected_field + 1) % state.config_fields.len();
                        }
                    }
                    KeyCode::BackTab => {
                        if state.editing_field {
                            if state.selected_field == 0 {
                                state.selected_field = state.config_fields.len() - 1;
                            } else {
                                state.selected_field -= 1;
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
    }

    terminal.clear()?;
    Ok(())
}

struct MenuState<'a> {
    selected_field: usize,
    editing_field: bool,
    unsaved_changes: bool,
    scroll_offset: usize,
    required_config: &'a ModelRequiredConfig,
    config_fields: Vec<String>,
    config_widgets: Vec<String>,
}
fn present_menu(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, state: &mut MenuState) -> Result<(), Box<dyn Error>> {
    terminal.draw(|frame| {
        let size = frame.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3), // View Model title
                Constraint::Percentage(100),
            ])
            .split(size);
        let block = Block::default()
            .borders(Borders::ALL)
            .title("View Model")
            .style(Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD));
        frame.render_widget(block, chunks[0]);

        let h_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
            .split(chunks[1]);

        // Actions on left
        let actions = if state.editing_field {
            vec![
                "[ESC] Stop Editing",
            ]
        } else if state.unsaved_changes {
            vec![
                "[E] Edit",
                "[B] Back",
                "[S] Save",
            ]
        } else {
            vec![
                "[E] Edit",
                "[B] Back",
            ]
        };
        let actions_list = StatefulList::new(actions.into_iter().map(ListItem::new).collect::<Vec<_>>());
        let actions_widget = List::new(actions_list.items.clone())
                .block(Block::default().title("Actions").borders(Borders::ALL))
                .style(Style::default().fg(Color::White));
        frame.render_widget(actions_widget, h_chunks[0]);

        // Further split right side vertically
        let v_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
            .split(h_chunks[1]);

        // Top right: Model static fields
        let formatted_string = format!("{} ({})", state.required_config.name, state.required_config.id);
        let question = Paragraph::new(Spans::from(vec![Span::raw(formatted_string)]))
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center)
            .block(Block::default()
                .border_style(Style::default().fg(Color::White))
                .borders(Borders::ALL)
                .title(Span::styled("\nModel", Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD))
            ));
        frame.render_widget(question, v_chunks[0]);

        // Bottom right: Model dynamic fields
        let num_displayed_fields = std::cmp::min(5, state.config_fields.len() - state.scroll_offset);


        let config_block = Block::default().title("Config").borders(Borders::ALL);
        frame.render_widget(config_block, v_chunks[1]);

        // Formatting dynamic properties
        let dynamic_properties = state.config_fields.iter()
            .skip(state.scroll_offset)
            .take(num_displayed_fields)
            .zip(state.config_widgets.iter().skip(state.scroll_offset).take(num_displayed_fields))
            .enumerate()
            .map(|(i, field)| {
                let is_selected = state.scroll_offset + i == state.selected_field;
                let field_style = if state.editing_field && is_selected {
                    Style::default().fg(Color::Black)
                } else {
                    Style::default().fg(Color::White)
                };
                let value_style = if state.editing_field && is_selected {
                    Style::default().fg(Color::Black).bg(Color::White)
                } else {
                    Style::default().fg(Color::White).bg(Color::Blue)
                };
                let field_span = Span::styled(format!("{:<15}", field.0), field_style);
                let value_span = Span::styled(format!("{:<25}", state.config_widgets[state.scroll_offset + i]), value_style);
                let spans = Spans::from(vec![field_span, Span::raw(" "), value_span]);
                ListItem::new(spans)
            })
            .collect::<Vec<_>>();

        // Further bottom right horizontally 
        let bottom_right_h_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(4)
            .constraints([Constraint::Percentage(33), Constraint::Percentage(34), Constraint::Percentage(33)].as_ref())
            .split(v_chunks[1]);

        // List widget with centered content
        let properties_widget = List::new(dynamic_properties)
            .block(Block::default().borders(Borders::NONE).title("Configuration Parameters").title_alignment(Alignment::Center))
            .style(Style::default().fg(Color::White))
            .start_corner(Corner::TopLeft);

        frame.render_widget(properties_widget, bottom_right_h_chunks[1]);
    })?;
    Ok(())
}


fn present_confirmation(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<bool, Box<dyn Error>>{
    let mut confirmed = false;
    loop {
        terminal.draw(|frame| {
            let size = frame.size();

            let text = "Save changes? [Y/N]";
            let block = Block::default().borders(Borders::ALL);
            let paragraph = Paragraph::new(text)
                .style(Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD))
                .block(block);
            frame.render_widget(paragraph, size);
        })?;

        match event::read()? {
            Event::Key(event) => match event.kind {
                KeyEventKind::Press => match event.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') => {
                        confirmed = true;
                        break;
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') => {
                        break;
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
    }
    terminal.clear()?;
    Ok(confirmed)
}

fn present_abort_confirmation(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<bool, Box<dyn Error>> {
    let text = "Unsaved changes will be aborted. Do you want to continue? [Y/N]: ";
    let mut confirmed = false;

    loop {
        terminal.draw(|frame| {
            let size = frame.size();
            let block = Block::default()
                .borders(Borders::ALL)
                .title("Abort Confirmation")
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
            // frame.render_widget(block, size);

            let text_block = Paragraph::new(Spans::from(vec![Span::raw(text)]))
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Left)
                .block(block);
            frame.render_widget(text_block, size);
        })?;

        match event::read()? {
            Event::Key(event) => match event.kind {
                KeyEventKind::Press => match event.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') => {
                        confirmed = true;
                        break;
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') => {
                        break;
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
    }

    terminal.clear()?;
    Ok(confirmed)
}