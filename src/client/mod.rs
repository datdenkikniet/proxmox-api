use std::time::{Duration, Instant};

use parking_lot::RwLock;
use reqwest::{blocking::RequestBuilder, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    access::{Ticket, TicketResponse},
    cluster::NextId,
    pools::PoolData,
};

mod nodes;
pub use nodes::NodeClient;

mod test;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    EncounteredErrors(serde_json::Value),
    ResponseWasNotString,
    DecodingFailed(String, serde_json::Error),
    UnknownFailure(StatusCode),
    Other(&'static str),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}

#[derive(Debug)]
struct AuthState {
    auth_ticket: Option<String>,
    auth_ticket_time: Instant,
    csrf_token: Option<String>,
    api_token: Option<String>,
}

#[derive(Debug)]
pub struct Client {
    client: reqwest::blocking::Client,
    host: String,

    user: String,
    realm: String,

    auth_state: RwLock<AuthState>,
}

impl Client {
    fn client() -> reqwest::blocking::Client {
        reqwest::blocking::ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap()
    }

    fn new_empty(host: &str, user: &str, realm: &str) -> Self {
        Self {
            client: Self::client(),
            host: host.to_string(),
            user: user.into(),
            realm: realm.into(),
            auth_state: RwLock::new(AuthState {
                auth_ticket: None,
                csrf_token: None,
                auth_ticket_time: Instant::now(),
                api_token: None,
            }),
        }
    }

    pub fn new_with_api_token(
        host: &str,
        user: &str,
        realm: &str,
        token_id: &str,
        token: &str,
    ) -> Result<Self, Error> {
        let me = Self::new_empty(host, user, realm);

        // PVEAPIToken=USER@REALM!TOKENID=UUID
        let api_token = format!("{user}@{realm}!{token_id}={token}");
        me.login(&api_token)?;
        me.auth_state.write().api_token = Some(api_token);

        Ok(me)
    }

    pub fn new(host: &str, user: &str, realm: &str, password: &str) -> Result<Self, Error> {
        let me = Self::new_empty(host, user, realm);

        me.login(password)?;

        Ok(me)
    }

    fn route(&self, path: &str) -> String {
        format!("{}/api2/json{}", self.host, path)
    }

    fn append_headers(&self, request: RequestBuilder) -> RequestBuilder {
        let auth_state = self.auth_state.read();

        let request = if let Some(auth_ticket) = &auth_state.auth_ticket {
            request.header("Cookie", format!("PVEAuthCookie={auth_ticket}"))
        } else {
            request
        };

        let request = if let Some(csrf) = &auth_state.csrf_token {
            request.header("CSRFPreventionToken", csrf)
        } else {
            request
        };

        let request = if let Some(api_token) = &auth_state.api_token {
            request.header("Authorization", api_token)
        } else {
            request
        };

        request
    }

    fn login(&self, password: &str) -> Result<(), Error> {
        let user = self.user.to_string();
        let realm = self.realm.to_string();
        let request = Ticket::new(&user, &realm, password);

        let csrf_details: TicketResponse = self.post("/access/ticket", &request)?;

        let mut auth_state = self.auth_state.write();

        auth_state.auth_ticket_time = Instant::now();
        let ticket = csrf_details
            .auth_ticket
            .ok_or(Error::Other("Missing ticket from access response!"))?;
        auth_state.auth_ticket = Some(format!("{ticket}"));

        auth_state.csrf_token = Some(
            csrf_details
                .csrf_token
                .ok_or(Error::Other("Missing CSRF token from access response!"))?,
        );

        Ok(())
    }

    /// Call this at least once every two hours.
    ///
    /// The ticket will automatically refresh if the last auth ticket was obtained more
    /// than an hour ago, or if `force` is set to `true`.
    pub fn refresh_auth_ticket(&self, force: bool) -> Result<(), Error> {
        log::trace!("Checking whether auth ticket should be refreshed (force: {force})");

        let auth_ticket = self
            .auth_state
            .read()
            .auth_ticket
            .as_ref()
            .expect("Cannot refresh auth ticket without having logged in previously.")
            .to_string();

        if force || self.auth_state.read().auth_ticket_time.elapsed() > Duration::from_secs(60 * 60)
        {
            // TODO: lock auth state during entire login operation to avoid
            // Time Of Check Time Of Use barriers
            log::debug!("Refreshing auth ticket.");
            self.login(&auth_ticket)?;
        }

        Ok(())
    }

    fn post<T, R, S>(&self, path: S, body: &T) -> Result<R, Error>
    where
        T: Serialize,
        R: DeserializeOwned,
        S: AsRef<str>,
    {
        log::debug!("POST {}", path.as_ref());

        let body = serde_urlencoded::to_string(body).unwrap();

        let request = self.client.post(self.route(path.as_ref())).body(body);

        let response = self.append_headers(request).send()?;
        let response_status = response.status();
        let json_data = response.bytes()?;
        let json_str = std::str::from_utf8(&json_data).map_err(|_| Error::ResponseWasNotString)?;

        log::debug!("JSON response: {json_str}");

        let result: Response<R> = serde_json::from_str(json_str)
            .map_err(|e| Error::DecodingFailed(json_str.into(), e))?;

        if let Some(data) = result.data {
            Ok(data)
        } else if let Some(errors) = result.errors {
            Err(Error::EncounteredErrors(errors))
        } else {
            Err(Error::UnknownFailure(response_status))
        }
    }

    fn get<R, S, Q>(&self, path: S, query: &Q) -> Result<R, Error>
    where
        R: DeserializeOwned,
        S: AsRef<str>,
        Q: Serialize,
    {
        log::debug!("GET {}", path.as_ref());

        let request = self.client.get(self.route(path.as_ref())).query(query);

        let response = self.append_headers(request).send()?;
        let response_status = response.status();
        let json_data = response.bytes()?;
        let json_str = std::str::from_utf8(&json_data).map_err(|_| Error::ResponseWasNotString)?;

        log::debug!("JSON response: {json_str}");

        let result: Response<R> = serde_json::from_str(json_str)
            .map_err(|e| Error::DecodingFailed(json_str.into(), e))?;

        if let Some(data) = result.data {
            Ok(data)
        } else if let Some(errors) = result.errors {
            Err(Error::EncounteredErrors(errors))
        } else {
            Err(Error::UnknownFailure(response_status))
        }
    }

    pub fn node<T>(&self, node: T) -> NodeClient
    where
        T: AsRef<str>,
    {
        NodeClient::new(self, node)
    }
}

// Cluster
impl Client {
    pub fn cluster_nextid(&self) -> Result<NextId, Error> {
        self.get("/cluster/nextid", &())
    }

    pub fn cluster_status(&self) -> Result<serde_json::Value, Error> {
        self.get("/cluster/status", &())
    }
}

// Pool
impl Client {
    pub fn pool_info(&self, pool_id: &str) -> Result<PoolData, Error> {
        self.get(&format!("/pools/{pool_id}"), &())
    }
}

#[derive(Debug, Deserialize)]
pub struct Response<T> {
    pub data: Option<T>,
    pub errors: Option<serde_json::Value>,
}
