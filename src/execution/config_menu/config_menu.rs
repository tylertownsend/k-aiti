use std::io;
use crossterm::{
    event::{self, Event as CEvent, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};

use crate::execution::config_menu::view_models;

use crate::config::Config;

#[derive(PartialEq)]
enum MenuItem {
    SelectMode,
    ViewModels,
    // AddRemoveModels,
    Quit,
}

pub async fn run_config_menu(config: &mut Config) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?; // Clear the screen before rendering the menu
    terminal.hide_cursor()?; // Hide the cursor

    let menu_items = [
        MenuItem::SelectMode,
        MenuItem::ViewModels,
        // MenuItem::AddRemoveModels,
        // MenuItem::ConfigureModels,
        MenuItem::Quit,
    ];

    let mut running = true;

    let mut selected_item = 0;
    while running {
        terminal.draw(|f| {
            let size = f.size();
            // let layout = Layout::default()
            //     .direction(Direction::Vertical)
            //     .constraints([Constraint::Percentage(100)].as_ref())
            //     .split(size);

            let menu_area = centered_rect(40, 50, size);

            let menu_widget = menu_list_widget(&menu_items, selected_item);
            f.render_widget(menu_widget, menu_area);
        })?;

        match event::read()? {
            CEvent::Key(event) => match event.code {
                KeyCode::Up => {
                    if selected_item > 0 {
                        selected_item -= 1;
                    }
                    // models_list.next();
                }
                KeyCode::Down => {
                    if selected_item < menu_items.len() - 1 {
                        selected_item += 1;
                    }
                }
                KeyCode::Char('q') | KeyCode::Char('Q') => {
                    running = false;
                }
                KeyCode::Enter => match menu_items[selected_item] {
                    MenuItem::Quit => running = false,
                    MenuItem::ViewModels => {
                        view_models::draw_view_models(&mut terminal, config)?;
                    }
                    // Handle other menu items here
                    _ => {}
                },
                KeyCode::Esc => running = false,
                _ => {}
            },
            _ => {}
        }
    }

    disable_raw_mode()?;
    terminal.clear()?; // Clear the screen before exiting
    terminal.show_cursor()?; // Show the cursor
    Ok(())
}

fn menu_list_widget<'a>(
    items: &[MenuItem],
    selected_item: usize,
) -> List<'a> {
    let menu_items: Vec<_> = items
        .iter()
        .map(|item| {
            let (label, _option) = match item {
                MenuItem::SelectMode => ("[1] Select Mode", "SelectMode"),
                MenuItem::ViewModels => ("[2] View Models", "ViewModels"),
                // MenuItem::AddRemoveModels => (
                //     "[3] Add/Remove Models",
                //     "AddRemoveModels",
                // ),
                // MenuItem::ConfigureModels => (
                //     "[4] Configure Models",
                //     "ConfigureModels",
                // ),
                MenuItem::Quit => ("[Q] Quit", "Quit"),
            };

            let style = if items[selected_item] == *item {
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
                .title("AI Terminal v1.0.0"),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let width = std::cmp::min(width, area.width);
    let height = std::cmp::min(height, area.height);
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 2;
    Rect::new(area.left() + x, area.top() + y, width, height)
}