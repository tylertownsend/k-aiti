use crossterm::event::{self, Event as CEvent, KeyCode};
use std::{io, error::Error, num};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, List},
    Terminal,
};

use crate::config::Model;


// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct Model {
//     id: String,
//     name: String,
//     model_path: String,
//     config: ModelConfig,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct ModelConfig {
//     max_tokens: u32,
//     temperature: f64,
//     top_p: f64,
// }

pub fn view(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, c_model: &mut Model) -> Result<(), Box<dyn Error>> {
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
    let mut config_widget: Vec<String> = config_fields
        .iter()
        .map(|field| {
            get_config_value_as_string(&config[field])
        })
        .collect();

    let mut selected_field = 0;
    let mut editing_field = false;
    let mut scroll_offset = 0;

    loop {
        terminal.draw(|frame| {
            let size = frame.size();
            let block = Block::default()
                .borders(Borders::ALL)
                .title("View Model")
                .style(Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD));
            frame.render_widget(block, size);
    

            // -----------------------------
            // Content on top of title block
            // -----------------------------
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(15),
                    Constraint::Length(3),
                ])
                .split(size);

            // static fields
            let formatted_string = format!("{} ({})", required_config.name, required_config.id);
            let question =Paragraph::new(Spans::from(vec![Span::raw(formatted_string)]))
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Left)
                .block(Block::default()
                    .border_style(Style::default().fg(Color::White))
                    .borders(Borders::ALL)
                    .title(Span::styled("Model", Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD))
                ));
            frame.render_widget(question, chunks[0]);


            // dynamic fields
            let mut field_list = Vec::new();
            let num_displayed_fields = std::cmp::min(5, config_fields.len() - scroll_offset);
            for (i, (label, input_widget)) in config_fields.iter()
                                                                            .skip(scroll_offset)
                                                                            .take(num_displayed_fields)
                                                                            .zip(config_widget
                                                                                .iter_mut()
                                                                                .skip(scroll_offset)
                                                                                .take(num_displayed_fields))
                                                                            .enumerate() {
                let input_block = Block::default()
                    .title(Span::styled(label, Style::default().fg(Color::White)))
                    .borders(Borders::ALL);
                let mut input_style = Style::default().fg(Color::White);
                if i + scroll_offset == selected_field {
                    input_style = input_style.bg(Color::DarkGray);
                }
                let input_text = if editing_field && i + scroll_offset == selected_field {
                    format!("{}|", input_widget.as_str())
                } else {
                    input_widget.to_string()
                };
                let input = Paragraph::new(input_text)
                    .style(input_style)
                    .block(input_block);
                field_list.push(input)
            }
            let field_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    std::iter::repeat(Constraint::Length(3))
                        .take(num_displayed_fields) // Limit to 5 displayed fields
                        .collect::<Vec<_>>()
                        .as_ref(),
                )
                .split(chunks[1]);

            for (i, field) in field_list.into_iter().enumerate() {
                frame.render_widget(field, field_chunks[i]);
            }

            // actions
            let action_text = "[S] Save  C] Cancel [B] Back";
            let action_span = Span::styled(action_text, Style::default().fg(Color::White));
            let action_block = Block::default().title(action_span).borders(Borders::ALL);
            frame.render_widget(action_block, chunks[2]);
        })?;

        match event::read()? {
            CEvent::Key(event) => match event.code {
                KeyCode::Char('s') | KeyCode::Char('S') => {
                    if !editing_field {
                        let confirmed = present_confirmation(terminal)?;
                        if confirmed {
                            // for (field, value) in config_fields.iter().zip(input_widgets.iter()) {
                            //     config[field] = serde_json::Value::String(value.clone());
                            // }
                        }
                    }
                }
                KeyCode::Char('c') | KeyCode::Char('C') => {
                    if !editing_field {
                        // input_widgets = config_fields
                        //     .iter()
                        //     .map(|field| {
                        //         config[field]
                        //             .as_str()
                        //             .unwrap_or_else(|| "Unknown value")
                        //             .to_string()
                        //     })
                        //     .collect();
                    }
                }
                KeyCode::Char('b') | KeyCode::Char('B') => {
                    if !editing_field {
                        break;
                    }
                }
                KeyCode::Up => {
                    if !editing_field {
                        if selected_field > 0 {
                            selected_field -= 1;
                        }
                        if selected_field < scroll_offset {
                            scroll_offset = selected_field;
                        }
                    }
                }
                KeyCode::Down => {
                    if !editing_field {
                        if selected_field < config_fields.len() - 1 {
                            selected_field += 1;
                        }
                        if selected_field >= scroll_offset + 5 {
                            scroll_offset = selected_field - 4;
                        }
                    }
                }
                KeyCode::Enter => {
                    if !editing_field && selected_field >= config_fields.len() {
                        editing_field = !editing_field;
                    }
                }
                KeyCode::Char(c) => {
                    if editing_field {
                        // input_widgets[selected_field].push(c);
                    }
                }
                KeyCode::Backspace => {
                    if editing_field {
                        // input_widgets[selected_field].pop();
                    }
                }
                KeyCode::Esc => {
                    if editing_field {
                        editing_field = false;
                    } else {
                        break;
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    terminal.clear()?;
    Ok(())
}

fn present_confirmation(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<bool, Box<dyn Error>>{
    let mut confirmed = false;
    loop {
        terminal.draw(|frame| {
            let size = frame.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

            let text = "Save changes? [Y/N]";
            let span = Span::styled(
                text,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            );
            let block = Block::default().title(span).borders(Borders::ALL);
            frame.render_widget(block, chunks[0]);
        })?;

        match event::read()? {
            CEvent::Key(event) => match event.code {
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
        }
    }
    terminal.clear()?;
    Ok(confirmed)
}