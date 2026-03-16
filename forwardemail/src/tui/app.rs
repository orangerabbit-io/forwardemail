use forwardemail_lib::client::Client;
use forwardemail_lib::models::account::Account;
use forwardemail_lib::models::alias::Alias;
use forwardemail_lib::models::domain::Domain;
use forwardemail_lib::models::email::Email;

#[derive(Debug, Clone, PartialEq)]
pub enum View {
    Dashboard,
    DomainList,
    DomainDetail(usize),
    AliasList(String),
    AliasDetail(String, usize),
    EmailList,
    EmailDetail(usize),
}

pub struct App {
    pub view: View,
    pub account: Option<Account>,
    pub domains: Vec<Domain>,
    pub aliases: Vec<Alias>,
    pub emails: Vec<Email>,
    pub selected: usize,
    pub should_quit: bool,
    pub error: Option<String>,
}

impl App {
    pub fn new() -> Self {
        App {
            view: View::Dashboard,
            account: None,
            domains: Vec::new(),
            aliases: Vec::new(),
            emails: Vec::new(),
            selected: 0,
            should_quit: false,
            error: None,
        }
    }

    pub fn load_dashboard(&mut self, client: &Client) {
        match client.get_json::<Account>("/v1/account") {
            Ok(account) => {
                self.account = Some(account);
                self.error = None;
            }
            Err(e) => self.error = Some(format!("{:#}", e)),
        }
    }

    pub fn load_domains(&mut self, client: &Client) {
        match client.get_json::<Vec<Domain>>("/v1/domains") {
            Ok(domains) => {
                self.domains = domains;
                self.selected = 0;
                self.error = None;
            }
            Err(e) => self.error = Some(format!("{:#}", e)),
        }
    }

    pub fn load_aliases(&mut self, client: &Client, domain: &str) {
        let path = format!("/v1/domains/{}/aliases", domain);
        match client.get_json::<Vec<Alias>>(&path) {
            Ok(aliases) => {
                self.aliases = aliases;
                self.selected = 0;
                self.error = None;
            }
            Err(e) => self.error = Some(format!("{:#}", e)),
        }
    }

    pub fn load_emails(&mut self, client: &Client) {
        match client.get_json::<Vec<Email>>("/v1/emails") {
            Ok(emails) => {
                self.emails = emails;
                self.selected = 0;
                self.error = None;
            }
            Err(e) => self.error = Some(format!("{:#}", e)),
        }
    }

    pub fn navigate_forward(&mut self, client: &Client) {
        match &self.view {
            View::Dashboard => {
                self.view = View::DomainList;
                self.load_domains(client);
            }
            View::DomainList => {
                if !self.domains.is_empty() {
                    self.view = View::DomainDetail(self.selected);
                }
            }
            View::DomainDetail(idx) => {
                let domain = self.domains[*idx].name.clone();
                self.view = View::AliasList(domain.clone());
                self.load_aliases(client, &domain);
            }
            View::AliasList(domain) => {
                if !self.aliases.is_empty() {
                    self.view = View::AliasDetail(domain.clone(), self.selected);
                }
            }
            View::EmailList => {
                if !self.emails.is_empty() {
                    self.view = View::EmailDetail(self.selected);
                }
            }
            _ => {}
        }
    }

    pub fn navigate_back(&mut self) {
        self.selected = 0;
        match &self.view {
            View::Dashboard => self.should_quit = true,
            View::DomainList | View::EmailList => self.view = View::Dashboard,
            View::DomainDetail(_) => self.view = View::DomainList,
            View::AliasList(domain) => {
                // Go back to the domain detail by finding the domain index
                let idx = self
                    .domains
                    .iter()
                    .position(|d| d.name == *domain)
                    .unwrap_or(0);
                self.view = View::DomainDetail(idx);
            }
            View::AliasDetail(domain, _) => {
                self.view = View::AliasList(domain.clone());
            }
            View::EmailDetail(_) => self.view = View::EmailList,
        }
    }

    pub fn list_len(&self) -> usize {
        match &self.view {
            View::DomainList => self.domains.len(),
            View::AliasList(_) => self.aliases.len(),
            View::EmailList => self.emails.len(),
            _ => 0,
        }
    }

    pub fn select_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn select_down(&mut self) {
        let len = self.list_len();
        if len > 0 && self.selected < len - 1 {
            self.selected += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let app = App::new();
        assert_eq!(app.view, View::Dashboard);
        assert!(!app.should_quit);
    }

    #[test]
    fn test_navigate_back_from_dashboard_quits() {
        let mut app = App::new();
        app.navigate_back();
        assert!(app.should_quit);
    }

    #[test]
    fn test_select_up_down() {
        let mut app = App::new();
        app.view = View::DomainList;
        app.domains = vec![
            Domain {
                name: "a.com".to_string(),
                plan: None,
                max_recipients_per_alias: None,
                smtp_port: None,
                has_adult_content_protection: None,
                has_phishing_protection: None,
                has_executable_protection: None,
                has_virus_protection: None,
                has_recipient_verification: None,
                retention_days: None,
                has_mx_record: None,
                has_txt_record: None,
                created_at: None,
                updated_at: None,
            },
            Domain {
                name: "b.com".to_string(),
                plan: None,
                max_recipients_per_alias: None,
                smtp_port: None,
                has_adult_content_protection: None,
                has_phishing_protection: None,
                has_executable_protection: None,
                has_virus_protection: None,
                has_recipient_verification: None,
                retention_days: None,
                has_mx_record: None,
                has_txt_record: None,
                created_at: None,
                updated_at: None,
            },
        ];
        assert_eq!(app.selected, 0);
        app.select_down();
        assert_eq!(app.selected, 1);
        app.select_down(); // at end, stays at 1
        assert_eq!(app.selected, 1);
        app.select_up();
        assert_eq!(app.selected, 0);
        app.select_up(); // at start, stays at 0
        assert_eq!(app.selected, 0);
    }
}
