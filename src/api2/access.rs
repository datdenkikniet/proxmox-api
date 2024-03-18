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
