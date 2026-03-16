use ratatui::prelude::*;
use ratatui::widgets::{Paragraph, Row, Table};

use crate::tui::app::App;

pub fn draw_list(f: &mut Frame, app: &App, area: Rect) {
    if app.aliases.is_empty() {
        let msg = Paragraph::new("  No aliases found.");
        f.render_widget(msg, area);
        return;
    }

    let header = Row::new(vec!["NAME", "RECIPIENTS", "ENABLED", "IMAP"])
        .style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan));

    let rows: Vec<Row> = app
        .aliases
        .iter()
        .enumerate()
        .map(|(i, a)| {
            let style = if i == app.selected {
                Style::default().bg(Color::DarkGray).fg(Color::White)
            } else {
                Style::default()
            };
            Row::new(vec![
                a.name.clone().unwrap_or("-".to_string()),
                a.recipients
                    .as_ref()
                    .map(|r| r.join(", "))
                    .unwrap_or("-".to_string()),
                a.is_enabled
                    .map(|b| if b { "yes" } else { "no" })
                    .unwrap_or("-")
                    .to_string(),
                a.has_imap
                    .map(|b| if b { "yes" } else { "no" })
                    .unwrap_or("-")
                    .to_string(),
            ])
            .style(style)
        })
        .collect();

    let widths = [
        Constraint::Percentage(25),
        Constraint::Percentage(40),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
    ];

    let table = Table::new(rows, widths).header(header);
    f.render_widget(table, area);
}

pub fn draw_detail(f: &mut Frame, app: &App, idx: usize, area: Rect) {
    let text = match app.aliases.get(idx) {
        Some(a) => {
            format!(
                "  ID:           {}\n  Name:         {}\n  Domain:       {}\n  Recipients:   {}\n  Enabled:      {}\n  IMAP:         {}\n  PGP:          {}\n  Description:  {}\n  Labels:       {}\n  Created:      {}\n  Updated:      {}",
                a.id,
                a.name.as_deref().unwrap_or("-"),
                a.domain.as_deref().unwrap_or("-"),
                a.recipients.as_ref().map(|r| r.join(", ")).unwrap_or("-".to_string()),
                a.is_enabled.map(|b| if b { "yes" } else { "no" }).unwrap_or("-"),
                a.has_imap.map(|b| if b { "yes" } else { "no" }).unwrap_or("-"),
                a.has_pgp.map(|b| if b { "yes" } else { "no" }).unwrap_or("-"),
                a.description.as_deref().unwrap_or("-"),
                a.labels.as_ref().map(|l| l.join(", ")).unwrap_or("-".to_string()),
                a.created_at.as_deref().unwrap_or("-"),
                a.updated_at.as_deref().unwrap_or("-"),
            )
        }
        None => "  Alias not found.".to_string(),
    };

    let paragraph = Paragraph::new(text).style(Style::default().fg(Color::White));
    f.render_widget(paragraph, area);
}
