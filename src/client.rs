use std::time::{Duration, Instant};

use reqwest::blocking::RequestBuilder;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use proxmox_api::access::{Ticket, TicketResponse};

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Other(&'static str),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}

#[derive(Debug)]
pub struct Client {
    client: reqwest::blocking::Client,
    host: String,

    auth_ticket_cookie: Option<String>,

    ticket_time: Instant,
    csrf_token: Option<String>,

    api_token: Option<String>,
}

impl Client {
    fn client() -> reqwest::blocking::Client {
        reqwest::blocking::ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap()
    }

    fn new_empty(host: &str) -> Self {
        Self {
            client: Self::client(),
            host: host.to_string(),
            auth_ticket_cookie: None,
            csrf_token: None,
            ticket_time: Instant::now(),
            api_token: None,
        }
    }

    pub fn new_with_api_token(
        host: &str,
        user: &str,
        realm: &str,
        token_id: &str,
        token: &str,
    ) -> Self {
        let mut me = Self::new_empty(host);

        // PVEAPIToken=USER@REALM!TOKENID=UUID
        me.api_token = Some(format!("{user}@{realm}!{token_id}={token}"));

        me
    }

    pub fn new(host: &str, user: &str, realm: &str, password: &str) -> Result<Self, Error> {
        let mut me = Self::new_empty(host);

        me.login(user, realm, password)?;

        Ok(me)
    }

    pub fn cluster_status(&mut self) -> Result<Response<serde_json::Value>, reqwest::Error> {
        Ok(self.get("/cluster/status")?)
    }

    fn route(&self, path: &str) -> String {
        format!("{}/api2/json{}", self.host, path)
    }

    fn append_headers(&self, request: RequestBuilder) -> RequestBuilder {
        let request = if let Some(cookie) = &self.auth_ticket_cookie {
            request.header("Cookie", cookie)
        } else {
            request
        };

        let request = if let Some(csrf) = &self.csrf_token {
            request.header("CSRFPreventionToken", csrf)
        } else {
            request
        };

        let request = if let Some(api_token) = &self.api_token {
            request.header("Authorization", api_token)
        } else {
            request
        };

        request
    }

    fn login(&mut self, user: &str, realm: &str, password: &str) -> Result<(), Error> {
        let request = Ticket::new(user, realm, password);

        let csrf_details: Response<TicketResponse> = self.post("/access/ticket", &request)?;
        let csrf_details = match csrf_details.data {
            Some(data) => data,
            None => panic!("{:?}", csrf_details.errors),
        };

        self.ticket_time = Instant::now();
        let ticket = csrf_details
            .auth_ticket
            .ok_or(Error::Other("Missing ticket from access response!"))?;
        self.auth_ticket_cookie = Some(format!("PVEAuthCookie={ticket}"));

        self.csrf_token = Some(
            csrf_details
                .csrf_token
                .ok_or(Error::Other("Missing CSRF token from access response!"))?,
        );

        Ok(())
    }

    fn ticket_refresh_check(&mut self) -> Result<(), reqwest::Error> {
        if self.ticket_time.elapsed() > Duration::from_secs(60 * 60) {
            // self.login(self.user, realm, password)?;
        }

        Ok(())
    }

    fn post<T, R>(&mut self, path: &str, body: &T) -> Result<Response<R>, reqwest::Error>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let body = serde_urlencoded::to_string(body).unwrap();

        let request = self.client.post(self.route(path)).body(body);

        let response = self.append_headers(request).send()?;

        response.json()
    }

    fn get<R>(&mut self, path: &str) -> Result<Response<R>, reqwest::Error>
    where
        R: DeserializeOwned,
    {
        let request = self.client.get(self.route(path));

        self.append_headers(request).send()?.json()
    }
}

#[derive(Debug, Deserialize)]
pub struct Response<T> {
    pub data: Option<T>,
    pub errors: Option<serde_json::Value>,
}
