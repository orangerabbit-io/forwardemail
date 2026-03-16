use ratatui::prelude::*;
use ratatui::widgets::{Paragraph, Row, Table};

use crate::tui::app::App;

pub fn draw_list(f: &mut Frame, app: &App, area: Rect) {
    if app.domains.is_empty() {
        let msg = Paragraph::new("  No domains found.");
        f.render_widget(msg, area);
        return;
    }

    let header = Row::new(vec!["DOMAIN", "PLAN", "MX", "TXT", "CREATED"])
        .style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan));

    let rows: Vec<Row> = app
        .domains
        .iter()
        .enumerate()
        .map(|(i, d)| {
            let style = if i == app.selected {
                Style::default().bg(Color::DarkGray).fg(Color::White)
            } else {
                Style::default()
            };
            Row::new(vec![
                d.name.clone(),
                d.plan.clone().unwrap_or("-".to_string()),
                d.has_mx_record
                    .map(|b| if b { "yes" } else { "no" })
                    .unwrap_or("-")
                    .to_string(),
                d.has_txt_record
                    .map(|b| if b { "yes" } else { "no" })
                    .unwrap_or("-")
                    .to_string(),
                d.created_at.clone().unwrap_or("-".to_string()),
            ])
            .style(style)
        })
        .collect();

    let widths = [
        Constraint::Percentage(30),
        Constraint::Percentage(20),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(30),
    ];

    let table = Table::new(rows, widths).header(header);
    f.render_widget(table, area);
}

pub fn draw_detail(f: &mut Frame, app: &App, idx: usize, area: Rect) {
    let text = match app.domains.get(idx) {
        Some(d) => {
            format!(
                "  Domain:       {}\n  Plan:         {}\n  MX Record:    {}\n  TXT Record:   {}\n  SMTP Port:    {}\n  Retention:    {} days\n  Max Rcpts:    {}\n  Created:      {}\n  Updated:      {}",
                d.name,
                d.plan.as_deref().unwrap_or("-"),
                d.has_mx_record.map(|b| if b { "yes" } else { "no" }).unwrap_or("-"),
                d.has_txt_record.map(|b| if b { "yes" } else { "no" }).unwrap_or("-"),
                d.smtp_port.as_deref().unwrap_or("-"),
                d.retention_days.map(|r| r.to_string()).unwrap_or("-".to_string()),
                d.max_recipients_per_alias.map(|m| m.to_string()).unwrap_or("-".to_string()),
                d.created_at.as_deref().unwrap_or("-"),
                d.updated_at.as_deref().unwrap_or("-"),
            )
        }
        None => "  Domain not found.".to_string(),
    };

    let paragraph = Paragraph::new(text).style(Style::default().fg(Color::White));
    f.render_widget(paragraph, area);
}
