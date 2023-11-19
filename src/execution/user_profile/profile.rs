use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, ListItem, List, Paragraph, Wrap},
    Terminal,
};

use super::super::ui::StatefulList;

// intro
pub fn draw_intro(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<(), Box<dyn std::error::Error>> {
    terminal.draw(|f| {
        let size = f.size();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3), // title block
                    Constraint::Percentage(80), // rest for content,
                    Constraint::Percentage(20)
                ]
                .as_ref(),
            )
            .split(size);

        // Title block
        let block = Block::default().title("CLI - Profile Setup")
            .borders(Borders::ALL);
        f.render_widget(block, chunks[0]);

        let intro_text = "Welcome to the OpenAI CLI setup!\nFollow the instructions to configure your user profile.";
        let intro = Paragraph::new(intro_text)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center)
            .block(Block::default()
                .borders(Borders::ALL)
                .title(Span::styled("Introduction", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)))
            );
        f.render_widget(intro, chunks[1]);

        let prompt_text = "Press Enter to continue...";
        let prompt = Paragraph::new(prompt_text)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center)
            .block(Block::default()
                .borders(Borders::ALL)
            );
        f.render_widget(prompt, chunks[2]);
    })?;

    Ok(())
}

pub fn draw_profile_confirmation_screen(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    api_key: &str,
    choices_list: &mut StatefulList<ListItem>,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal.draw(|f| {
        let size = f.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3), // title
                    Constraint::Percentage(80), // content
                    Constraint::Length(3), // action
                ]
                .as_ref(),
            )
            .split(size);

        let title_block = Block::default().borders(Borders::ALL).title("CLI - Profile Setup");
        f.render_widget(title_block, chunks[0]);

        // Contennt Block
        let block = Block::default().title("Profile Confirmation").borders(Borders::ALL);
        f.render_widget(block, chunks[1]);

        let content_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(3),
                    Constraint::Length(1),
                    Constraint::Length(5),
                    Constraint::Length(3) // fill up remaining area
                ]
                .as_ref(),
            )
            .split(chunks[1]);

        let review_prompt = Paragraph::new("Review your profile information:")
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left);
        f.render_widget(review_prompt, content_chunks[0]);

        let api_key_display = format!("API Key: {}", api_key);
        let api_key_paragraph = Paragraph::new(api_key_display)
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Left)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(api_key_paragraph, content_chunks[1]);

        let looks_good_prompt = Paragraph::new("Looks good?")
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center);
        f.render_widget(looks_good_prompt, content_chunks[2]);

        let h_chunks = Layout::default()
            .direction(Direction::Horizontal)
            // .margin(4)
            .constraints([Constraint::Percentage(33), Constraint::Percentage(34), Constraint::Percentage(33)].as_ref())
            .split(content_chunks[3]);

        let choices_widget = List::new(choices_list.items.clone())
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().bg(Color::DarkGray).fg(Color::White))
            .highlight_symbol("> ");
        f.render_stateful_widget(choices_widget, h_chunks[1], &mut choices_list.state);

        let action_text = "[Enter] Continue [B] Back";
        let action_span = Span::styled(action_text, Style::default().fg(Color::LightGreen));
        let action_block = Block::default().title(action_span).borders(Borders::ALL);
        f.render_widget(action_block, chunks[2]);
    })?;

    Ok(())
}

pub fn draw_profile_setup_complete_screen(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
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
                    Constraint::Percentage(20), // Actions
                ]
                .as_ref(),
            )
            .split(size);

        let block = Block::default().title("CLI - Profile Setup").borders(Borders::ALL);
        f.render_widget(block, chunks[0]);

        let completion_text = "Profile setup is now complete. Enjoy using the OpenAI CLI!";

        let completion_paragraph = Paragraph::new(completion_text)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL)); // Add borders to the content block
        f.render_widget(completion_paragraph, chunks[1]);

        let action_text = "[Enter] Finish";
        let action_span = Span::styled(action_text, Style::default().fg(Color::LightGreen));
        let action_block = Block::default().title(action_span).borders(Borders::ALL);
        f.render_widget(action_block, chunks[2]);
    })?;

    Ok(())
}