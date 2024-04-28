// Hardcoded auth stuff so we don't have to activate all of `access`.

use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Ticket<'a> {
    #[serde(rename = "username")]
    user: &'a str,
    realm: &'a str,
    password: &'a str,
}

impl<'a> Ticket<'a> {
    pub fn new(user: &'a str, realm: &'a str, password: &'a str) -> Self {
        Self {
            user,
            realm,
            password,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TicketResponse {
    pub username: String,
    #[serde(rename = "CSRFPreventionToken")]
    pub csrf_token: Option<String>,
    #[serde(rename = "clustername")]
    pub cluster_name: Option<String>,
    #[serde(rename = "ticket")]
    pub auth_ticket: Option<String>,
}

#[derive(Clone, Debug)]
pub struct AuthState {
    inner: Arc<RwLock<Inner>>,
}

impl AuthState {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(Inner::new())),
        }
    }

    /// Set the API token.
    pub fn set_api_token(&self, user: &str, realm: &str, token_id: &str, token: &str) {
        // PVEAPIToken=USER@REALM!TOKENID=UUID
        self.inner.write().api_token = Some(format!("{user}@{realm}!{token_id}={token}"));
    }

    /// Set the CSRF and auth ticket
    pub fn set_csrf(&self, ticket: String, csrf: String) {
        let mut inner = self.inner.write();

        inner.auth_ticket = Some(ticket);
        inner.csrf_token = Some(csrf);
        inner.auth_ticket_time = Instant::now();
    }

    pub fn should_refresh(&self) -> bool {
        self.inner.read().auth_ticket_time.elapsed() > Duration::from_secs(60 * 60)
    }

    pub fn auth_ticket(&self) -> Option<String> {
        self.inner.read().auth_ticket.clone()
    }

    pub fn api_token(&self) -> Option<String> {
        self.inner.read().api_token.clone()
    }

    pub fn headers(&self) -> impl Iterator<Item = (&str, String)> + '_ {
        let inner = self.inner.read();

        let cookie = inner
            .auth_ticket
            .as_ref()
            .map(|v| ("Cookie", format!("PVEAuthCookie={v}")));

        let csrf = inner
            .csrf_token
            .as_ref()
            .map(|v| ("CSRFPreventionToken", v.to_string()));

        let api_token = inner
            .api_token
            .as_ref()
            .map(|v| ("Authorization", format!("PVEAPIToken={v}")));

        cookie
            .into_iter()
            .chain(csrf.into_iter())
            .chain(api_token.into_iter())
    }
}

#[derive(Debug)]
struct Inner {
    auth_ticket: Option<String>,
    auth_ticket_time: Instant,
    csrf_token: Option<String>,
    api_token: Option<String>,
}

impl Inner {
    pub fn new() -> Self {
        Self {
            auth_ticket: None,
            auth_ticket_time: Instant::now() - Duration::from_secs(24 * 60 * 60),
            csrf_token: None,
            api_token: None,
        }
    }
}
