use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

use crate::tui::app::App;

pub fn draw(f: &mut Frame, app: &App, area: Rect) {
    let text = match &app.account {
        Some(account) => {
            let email = account.email.as_deref().unwrap_or("-");
            let name = match (&account.given_name, &account.family_name) {
                (Some(g), Some(fam)) => format!("{} {}", g, fam),
                (Some(g), None) => g.clone(),
                (None, Some(fam)) => fam.clone(),
                (None, None) => "-".to_string(),
            };
            let plan = account.plan.as_deref().unwrap_or("-");
            let created = account.created_at.as_deref().unwrap_or("-");

            format!(
                "  Email:    {}\n  Name:     {}\n  Plan:     {}\n  Created:  {}",
                email, name, plan, created
            )
        }
        None => "  Loading account...".to_string(),
    };

    let paragraph = Paragraph::new(text).style(Style::default().fg(Color::White));
    f.render_widget(paragraph, area);
}
