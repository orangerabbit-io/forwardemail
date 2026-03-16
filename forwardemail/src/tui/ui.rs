use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

use super::app::{App, View};
use super::views;

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // header
            Constraint::Min(0),   // content
            Constraint::Length(1), // footer
        ])
        .split(f.area());

    // Header with breadcrumb
    let breadcrumb = match &app.view {
        View::Dashboard => "Forward Email > Dashboard".to_string(),
        View::DomainList => "Forward Email > Domains".to_string(),
        View::DomainDetail(idx) => format!(
            "Forward Email > Domains > {}",
            app.domains
                .get(*idx)
                .map(|d| d.name.as_str())
                .unwrap_or("?")
        ),
        View::AliasList(domain) => {
            format!("Forward Email > Domains > {} > Aliases", domain)
        }
        View::AliasDetail(domain, _) => {
            format!(
                "Forward Email > Domains > {} > Aliases > Detail",
                domain
            )
        }
        View::EmailList => "Forward Email > Emails".to_string(),
        View::EmailDetail(_) => "Forward Email > Emails > Detail".to_string(),
    };
    let header = Paragraph::new(breadcrumb)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    f.render_widget(header, chunks[0]);

    // Content
    if let Some(err) = &app.error {
        let error =
            Paragraph::new(format!("Error: {}", err)).style(Style::default().fg(Color::Red));
        f.render_widget(error, chunks[1]);
    } else {
        match &app.view {
            View::Dashboard => views::dashboard::draw(f, app, chunks[1]),
            View::DomainList => views::domains::draw_list(f, app, chunks[1]),
            View::DomainDetail(idx) => views::domains::draw_detail(f, app, *idx, chunks[1]),
            View::AliasList(_) => views::aliases::draw_list(f, app, chunks[1]),
            View::AliasDetail(_, idx) => views::aliases::draw_detail(f, app, *idx, chunks[1]),
            View::EmailList => views::emails::draw_list(f, app, chunks[1]),
            View::EmailDetail(idx) => views::emails::draw_detail(f, app, *idx, chunks[1]),
        }
    }

    // Footer with keybindings
    let footer_text =
        "q: quit | Esc: back | Enter: select | j/k: navigate | d: domains | e: emails";
    let footer = Paragraph::new(footer_text).style(Style::default().fg(Color::DarkGray));
    f.render_widget(footer, chunks[2]);
}
