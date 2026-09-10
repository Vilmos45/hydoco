use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs},
    Frame,
    prelude::*,
};

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    let area = frame.area();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // title
            Constraint::Length(2), // tabs
            Constraint::Min(1),    // content
        ])
        .split(area);

    render_title(frame, layout[0]);
    render_tabs(frame, layout[1], app.tab);
    render_content(frame, layout[2], app.tab);
}

fn render_title(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(" HyDoCo - Hyprland Dots Config ")
        .title_alignment(Alignment::Center)
        .borders(Borders::TOP)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Green));

    frame.render_widget(block, area);
}

fn render_tabs(frame: &mut Frame, area: Rect, tab: u8) {
    let block = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(Style::default().fg(Color::Green));

    let tabs_area = block.inner(area);

    frame.render_widget(block, area);

    let tabs = Tabs::new(vec![
        "Main",
        "Appearance",
        "Animations",
        "Windows",
        "Input",
        "Laptop",
    ])
    .select(tab as usize)
    .style(Style::default().fg(Color::White))
    .highlight_style(
        Style::default()
            .fg(Color::Black)
            .bg(Color::Magenta)
            .bold(),
    )
    .divider("│")
    .padding(" ", " ");

    frame.render_widget(tabs, tabs_area);
}


/// Render the tab content.
pub fn render_content(frame: &mut Frame, area: Rect, tab: u8) {
    let text = match tab {
        0 => main_page(),
        1 => appearance_page(),
        2 => animations_page(),
        3 => windows_page(),
        4 => input_page(),
        5 => laptop_page(),
        _ => unreachable!(),
    };
    let block = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(Block::bordered());
    frame.render_widget(block, area);
}

pub fn main_page() -> &'static str {
    return 
    "Press 'Esc', 'Ctrl-C' or 'q' to quit.\n\
    Press 'TAB' to switch between widgets.\n";
}

pub fn appearance_page() -> &'static str {
    return "Appearance settings go here.";
}

pub fn animations_page() -> &'static str {
    return "Animations settings go here.";
}

pub fn windows_page() -> &'static str {
    return "Windows settings go here.";
}

pub fn input_page() -> &'static str {
    return "Input settings go here.";
}

pub fn laptop_page() -> &'static str {
    return "Laptop settings go here.";
}