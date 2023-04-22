use crossterm::event::{self, Event as CEvent, KeyCode};
use std::{io, error::Error};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

use serde::{Deserialize, Serialize};

use super::{Model, ModelConfig};

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

pub fn view(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, model: &mut Model) -> Result<(), Box<dyn Error>> {

    match &mut model.config {
        ModelConfig::OpenAIGPT { max_tokens, temperature, top_p} => {
            let mut config = OpenAIGPT { max_tokens: *max_tokens, temperature: *temperature, top_p: *top_p };
            let required_config = ModelRequiredConfig {id: model.id.to_string(), name: model.name.to_string()};
            config = view_gpt_config(terminal, required_config, config)?;
            *max_tokens = config.max_tokens;
            *temperature = config.temperature;
            *top_p = config.top_p;
            Ok(())
        }
    }
}

struct ModelRequiredConfig {
    id: String,
    name: String,
}
struct OpenAIGPT { max_tokens: u32, temperature: f64, top_p: f64 }

fn view_gpt_config(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    required_config: ModelRequiredConfig,
    mut model_config: OpenAIGPT,
    ) -> Result<OpenAIGPT, Box<dyn Error>> {

    // Model fields
    // let mut id = String::from("chat_model");
    // let mut name = String::from("ChatGPT");
    // // let mut model_path = String::from("models/chat_gpt/");
    // let mut max_tokens = String::from("200");
    // let mut temperature = String::from("0.8");
    // let mut top_p = String::from("0.9");

    let mut selected_field = 0;
    let mut editing_field = false;

    // modify to use real json
    let mut input_widgets = [
        &mut required_config.id.to_string(),
        &mut required_config.name.to_string(),
        // &mut model_path,
        &mut model_config.max_tokens.to_string(),
        &mut model_config.temperature.to_string(),
        &mut model_config.top_p.to_string(),
    ];

    let labels = [
        "ID",
        "Name",
        // "Model Path",
        "Max Tokens",
        "Temperature",
        "Top P",
    ];

    loop {
        terminal.draw(|frame| {
            let size = frame.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);
        
            let title = "Edit Model";
            let title_span = Span::styled(
                title,
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            );
            let title_block = Block::default().title(title_span).borders(Borders::ALL);
            frame.render_widget(title_block, chunks[0]);
        

            for (i, (label, input_widget)) in labels.iter().zip(input_widgets.iter_mut()).enumerate() {
                let input_block = Block::default()
                    .title(Span::styled(*label, Style::default().fg(Color::Yellow)))
                    .borders(Borders::ALL);
                let mut input_style = Style::default().fg(Color::White);
                if i == selected_field {
                    input_style = input_style.bg(Color::DarkGray);
                }
                let input_text = if editing_field && i == selected_field {
                    format!("{}|", input_widget.as_str())
                } else {
                    input_widget.to_string()
                };
                let input = Paragraph::new(input_text)
                    .style(input_style)
                    .block(input_block);
                frame.render_widget(input, chunks[i + 1]);
            }
        
            let action_text = "[S] Save  [C] Cancel [B] Back";
            let action_span = Span::styled(action_text, Style::default().fg(Color::Yellow));
            let action_block = Block::default().title(action_span).borders(Borders::ALL);
            frame.render_widget(action_block, chunks[8]);
        })?;

        match event::read()? {
            CEvent::Key(event) => match event.code {
                KeyCode::Char('s') | KeyCode::Char('S') => {
                    if !editing_field {
                        let confirmed = present_confirmation(terminal)?;
                        if confirmed {
                            model_config.max_tokens = input_widgets[2].parse::<u32>()?;
                            model_config.temperature = input_widgets[3].parse::<f64>()?;
                            model_config.top_p =  input_widgets[4].parse::<f64>()?;
                        }
                    }
                }
                KeyCode::Char('c') | KeyCode::Char('C') => {
                    // Cancel logic
                    if !editing_field {
                        *input_widgets[2] = model_config.max_tokens.to_string();
                        *input_widgets[3] = model_config.temperature.to_string();
                        *input_widgets[4] = model_config.top_p.to_string();
                    }
                }
                KeyCode::Char('b') | KeyCode::Char('B') => {
                    // Go back to the previous screen
                    if !editing_field {
                        break;
                    }
                }
                KeyCode::Up => {
                    if !editing_field {
                        if selected_field > 0 {
                            selected_field -= 1;
                        }
                    }
                }
                KeyCode::Down => {
                    if !editing_field {
                        if selected_field < input_widgets.len() - 1 {
                            selected_field += 1;
                        }
                    }
                }
                KeyCode::Enter => {
                    editing_field = !editing_field;
                }
                KeyCode::Char(c) => {
                    if editing_field {
                        input_widgets[selected_field].push(c);
                    }
                }
                KeyCode::Backspace => {
                    if editing_field {
                        input_widgets[selected_field].pop();
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
    // let mut input_widgets = [
    //     &mut model.id,
    //     &mut model.name,
    //     // &mut model_path,
    //     &mut model_config.max_tokens.to_string(),
    //     &mut model_config.temperature.to_string(),
    //     &mut model_config.top_p.to_string(),
    // ];
    model_config.max_tokens = input_widgets[2].parse::<u32>()?;
    model_config.temperature = input_widgets[3].parse::<f64>()?;
    model_config.top_p =  input_widgets[4].parse::<f64>()?;

    terminal.clear()?;
    Ok(model_config)
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
                    .fg(Color::Yellow)
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