use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, ListItem, List, Paragraph, Wrap},
    Terminal,
};
use webbrowser;

use super::stateful_list::StatefulList;

pub fn draw_account_found(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    choices_list: &mut StatefulList<ListItem>,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal.draw(|f| {
        let size = f.size();

        // Define the layout constraints
        let constraints = [
            Constraint::Length(3), // Title block
            Constraint::Percentage(80), // Content block (will be split into three)
            Constraint::Length(3), // Action block
        ];

        // Create layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(constraints.as_ref())
            .split(size);

        // Create title block
        let block = Block::default().title("CLI - Profile Setup").borders(Borders::ALL);
        f.render_widget(block, chunks[0]);

        // Contennt Block
        let block = Block::default().title("Account Detection").borders(Borders::ALL);
        f.render_widget(block, chunks[1]);

        // Create content blocks
        let content_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Percentage(30),  // Adjust these percentages to
                Constraint::Length(2),  // control distance between blocks
                Constraint::Percentage(50)   // 'detected_prompt', 'use_api_key_prompt', 'choices_widget'
            ].as_ref())
            .split(chunks[1]);

        let detected_prompt = Paragraph::new("We have detected environment variable OPENAI_API_KEY in your system.")
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center);
        f.render_widget(detected_prompt, content_chunks[0]);

        let use_api_key_prompt = Paragraph::new("Would you like to use this as part of your openai account?")
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left);
        f.render_widget(use_api_key_prompt, content_chunks[1]);

        let choices_widget = List::new(choices_list.items.clone())
            .block(Block::default().borders(Borders::NONE))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().bg(Color::DarkGray).fg(Color::White))
            .highlight_symbol("> ");
        f.render_stateful_widget(choices_widget, content_chunks[2], &mut choices_list.state);

        // Create action block
        let action_text = "[Enter] Select [B] Back";
        let action_span = Span::styled(action_text, Style::default().fg(Color::LightGreen));
        let action_block = Block::default().title(action_span).borders(Borders::ALL);
        f.render_widget(action_block, chunks[2]);
    })?;

    Ok(())
}

pub fn draw_has_account_screen(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    choices_list: &mut StatefulList<ListItem>,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal.draw(|f| {
        let size = f.size();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3), // Title
                    Constraint::Percentage(80), // Content
                    Constraint::Length(3), // Actions
                ]
                .as_ref(),
            )
            .split(size);

        // Title Block
        let block = Block::default().borders(Borders::ALL).title("CLI - Profile Setup");
        f.render_widget(block, chunks[0]);

        // Content Block
        let question = Paragraph::new("Do you have an OpenAI account?")
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center)
            .block(Block::default().title(Span::styled("Profile Setup", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))).borders(Borders::ALL));
        f.render_widget(question, chunks[1]);

        let h_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(4)
            .constraints([Constraint::Percentage(33), Constraint::Percentage(34), Constraint::Percentage(33)].as_ref())
            .split(chunks[1]);

        let choices_widget = List::new(choices_list.items.clone())
            .block(Block::default().title("Select").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
            .highlight_symbol("> ");
        f.render_stateful_widget(choices_widget, h_chunks[1], &mut choices_list.state);

        // Action Block
        let action_text = "[Enter] Select [B] Back";
        let action_span = Span::styled(action_text, Style::default().fg(Color::LightGreen));
        let action_block = Block::default().title(action_span).borders(Borders::ALL);
        f.render_widget(action_block, chunks[2]);
    })?;

    Ok(())
}

pub fn draw_enter_openai_account_screen (
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    api_key: &str,
    editing_field: &mut bool,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal.draw(|f| {
        let size = f.size();

        // Define the layout constraints
        let constraints = [
            Constraint::Length(3), // Title block
            Constraint::Percentage(80), // Content block (will be split into three)
            Constraint::Length(3), // Action block
        ];

        // Create layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(constraints.as_ref())
            .split(size);

        // Title block
        let title = "CLI - Profile Setup";
        let title_span = Span::styled(
            title,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );
        let title_block = Block::default().title(title_span).borders(Borders::ALL);
        f.render_widget(title_block, chunks[0]);

        // Contennt Block
        let block = Block::default().title("API Key Information").borders(Borders::ALL);
        f.render_widget(block, chunks[1]);

        let content_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(2),
                    Constraint::Length(3),
                    Constraint::Length(3) // use to fill up remaining space in the content chunk
                ]
                .as_ref(),
            )
            .split(chunks[1]);

        let api_key_prompt = Paragraph::new("Enter your OpenAI API key:")
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left);
        f.render_widget(api_key_prompt, content_chunks[0]);

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
        f.render_widget(api_key_input, content_chunks[1]);

        // Actions block
        let action_text = " [Enter] Continue [B] Back [E] Edit";
        let action_span = Span::styled(action_text, Style::default().fg(Color::LightGreen));
        let action_block = Block::default().title(action_span).borders(Borders::ALL);
        f.render_widget(action_block, chunks[2]);
    })?;

    Ok(())
}

pub fn draw_create_openai_account_screen(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal.draw(|f| {
        let size = f.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3), // For the title block
                    Constraint::Percentage(90), // For the content block
                    Constraint::Length(3), // For the action block
                ]
                .as_ref(),
            )
            .split(size);

        // Title Block
        let title_block = Block::default()
            .borders(Borders::ALL)
            .title("CLI - Profile Setup");
        f.render_widget(title_block, chunks[0]);

        // Content Block
        let content_text = vec![
            Spans::from("Create an OpenAI account and obtain your API key."),
            Spans::from(""),
            Spans::from("If your browser hasn't opened, please use the following link:"),
            Spans::from("https://platform.openai.com/account/api-keys"),
        ];
        let content_block = Block::default().borders(Borders::ALL);
        let content_paragraph = Paragraph::new(content_text)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center)
            .block(content_block)
            .wrap(Wrap { trim: true });
        f.render_widget(content_paragraph, chunks[1]);

        // Action Block
        let action_text = "[Enter] Continue [B] Back";
        let action_span = Span::styled(action_text, Style::default().fg(Color::LightGreen));
        let action_block = Block::default().title(action_span).borders(Borders::ALL);
        f.render_widget(action_block, chunks[2]);
    })?;

    Ok(())
}

pub fn create_account() {
    let res = webbrowser::open("https://platform.openai.com/account/api-keys").is_err();
    if res {
        // encountered error
    }
}

pub fn draw_disclaimer_screen(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal.draw(|f| {
        let size = f.size();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3), // For Title
                    Constraint::Percentage(80), // For Content
                    Constraint::Length(3), // For Actions
                ]
                .as_ref(),
            )
            .split(size);

        // Title Block
        let title_block = Block::default().borders(Borders::ALL).title("OpenAI CLI - Profile Setup");
        f.render_widget(title_block, chunks[0]);

        // Content Block
        let disclaimer_text = "Always keep your API key secure \
                               and don't share it with others. Unauthorized \
                               usage may result in billing charges and other \
                               consequences.";
        let disclaimer_paragraph = Paragraph::new(disclaimer_text)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).title("Disclaimer")); // Add border to content block
        f.render_widget(disclaimer_paragraph, chunks[1]);

        // Action Block
        let action_text = "[Enter] Continue [B] Back";
        let action_span = Span::styled(action_text, Style::default().fg(Color::LightGreen));
        let action_block = Block::default().title(action_span).borders(Borders::ALL);
        f.render_widget(action_block, chunks[2]);
    })?;

    Ok(())
}
