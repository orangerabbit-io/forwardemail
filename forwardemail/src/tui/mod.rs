pub mod app;
pub mod ui;
pub mod views;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::prelude::*;
use std::io;

use app::App;
use forwardemail_lib::client::Client;

pub fn run(client: &Client) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    app.load_dashboard(client);

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q')
                | KeyCode::Char('c')
                    if key.modifiers.contains(KeyModifiers::CONTROL) =>
                {
                    app.should_quit = true;
                }
                KeyCode::Char('q') => app.should_quit = true,
                KeyCode::Esc | KeyCode::Backspace => app.navigate_back(),
                KeyCode::Enter | KeyCode::Char('l') => app.navigate_forward(client),
                KeyCode::Up | KeyCode::Char('k') => app.select_up(),
                KeyCode::Down | KeyCode::Char('j') => app.select_down(),
                KeyCode::Char('d') => {
                    app.view = app::View::DomainList;
                    app.load_domains(client);
                }
                KeyCode::Char('e') => {
                    app.view = app::View::EmailList;
                    app.load_emails(client);
                }
                _ => {}
            }
        }

        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
