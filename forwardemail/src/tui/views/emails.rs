use ratatui::prelude::*;
use ratatui::widgets::{Paragraph, Row, Table};

use crate::tui::app::App;

pub fn draw_list(f: &mut Frame, app: &App, area: Rect) {
    if app.emails.is_empty() {
        let msg = Paragraph::new("  No emails found.");
        f.render_widget(msg, area);
        return;
    }

    let header = Row::new(vec!["ID", "STATUS", "FROM", "TO", "SUBJECT"])
        .style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan));

    let rows: Vec<Row> = app
        .emails
        .iter()
        .enumerate()
        .map(|(i, e)| {
            let style = if i == app.selected {
                Style::default().bg(Color::DarkGray).fg(Color::White)
            } else {
                Style::default()
            };
            Row::new(vec![
                e.id.clone(),
                e.status.clone().unwrap_or("-".to_string()),
                e.from.clone().unwrap_or("-".to_string()),
                e.to.as_ref()
                    .map(|t| t.join(", "))
                    .unwrap_or("-".to_string()),
                e.subject.clone().unwrap_or("-".to_string()),
            ])
            .style(style)
        })
        .collect();

    let widths = [
        Constraint::Percentage(15),
        Constraint::Percentage(10),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    ];

    let table = Table::new(rows, widths).header(header);
    f.render_widget(table, area);
}

pub fn draw_detail(f: &mut Frame, app: &App, idx: usize, area: Rect) {
    let text = match app.emails.get(idx) {
        Some(e) => {
            format!(
                "  ID:       {}\n  Status:   {}\n  From:     {}\n  To:       {}\n  CC:       {}\n  BCC:      {}\n  Subject:  {}\n  Created:  {}\n  Updated:  {}",
                e.id,
                e.status.as_deref().unwrap_or("-"),
                e.from.as_deref().unwrap_or("-"),
                e.to.as_ref().map(|t| t.join(", ")).unwrap_or("-".to_string()),
                e.cc.as_ref().map(|c| c.join(", ")).unwrap_or("-".to_string()),
                e.bcc.as_ref().map(|b| b.join(", ")).unwrap_or("-".to_string()),
                e.subject.as_deref().unwrap_or("-"),
                e.created_at.as_deref().unwrap_or("-"),
                e.updated_at.as_deref().unwrap_or("-"),
            )
        }
        None => "  Email not found.".to_string(),
    };

    let paragraph = Paragraph::new(text).style(Style::default().fg(Color::White));
    f.render_widget(paragraph, area);
}
