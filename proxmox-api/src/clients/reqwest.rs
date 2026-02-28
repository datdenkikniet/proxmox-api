use std::sync::Arc;

use reqwest::{Method, StatusCode, blocking::RequestBuilder};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use super::base_access::{AuthState, Ticket, TicketResponse};

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

#[derive(Debug, Clone)]
pub struct Client {
    client: Arc<reqwest::blocking::Client>,
    host: String,

    user: String,
    realm: String,

    auth_state: AuthState,
}

impl Client {
    fn client() -> Arc<reqwest::blocking::Client> {
        Arc::new(
            reqwest::blocking::ClientBuilder::new()
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap(),
        )
    }

    fn new_empty(host: &str, user: &str, realm: &str) -> Self {
        Self {
            client: Self::client(),
            host: host.to_string(),
            user: user.into(),
            realm: realm.into(),
            auth_state: AuthState::new(),
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
        me.auth_state.set_api_token(user, realm, token_id, token);

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
        let mut request = request;
        for (k, v) in self.auth_state.headers() {
            request = request.header(k, v);
        }

        request
    }

    fn login(&self, password: &str) -> Result<(), Error> {
        let user = self.user.to_string();
        let realm = self.realm.to_string();
        let request = Ticket::new(&user, &realm, password);

        let csrf_details: TicketResponse =
            crate::client::Client::post(self, "/access/ticket", &request)?;

        let ticket = csrf_details
            .auth_ticket
            .ok_or(Error::Other("Missing ticket from access response!"))?;
        let csrf = csrf_details
            .csrf_token
            .ok_or(Error::Other("Missing CSRF token from access response!"))?;

        self.auth_state.set_csrf(ticket, csrf);

        Ok(())
    }

    /// Call this at least once every two hours.
    ///
    /// The ticket will automatically refresh if the last auth ticket was obtained more
    /// than an hour ago, or if `force` is set to `true`.
    pub fn refresh_auth_ticket(&self, force: bool) -> Result<(), Error> {
        log::trace!("Checking whether auth ticket should be refreshed (force: {force})");

        let auth_ticket = if let Some(ticket) = self.auth_state.auth_ticket() {
            ticket
        } else {
            if self.auth_state.api_token().is_none() {
                log::warn!(
                    "Tried to refresh auth ticket without existing auth ticket or API token."
                );
            }
            return Ok(());
        };

        if force || self.auth_state.should_refresh() {
            // TODO: lock auth state during entire login operation to avoid
            // Time Of Check Time Of Use barriers
            log::debug!("Refreshing auth ticket.");
            self.login(&auth_ticket)?;
        }

        Ok(())
    }
}

impl crate::client::Client for Client {
    type Error = Error;

    fn request_with_body_and_query<B, Q, R>(
        &self,
        method: crate::client::Method,
        path: &str,
        body: Option<&B>,
        query: Option<&Q>,
    ) -> Result<R, Error>
    where
        B: Serialize,
        Q: Serialize,
        R: DeserializeOwned,
    {
        let method = match method {
            crate::client::Method::Post => Method::POST,
            crate::client::Method::Get => Method::GET,
            crate::client::Method::Put => Method::PUT,
            crate::client::Method::Delete => Method::DELETE,
        };

        log::debug!("{} {}", method, path);

        let request = self.client.request(method, self.route(path.as_ref()));

        let request = if let Some(body) = body {
            let body = serde_urlencoded::to_string(body).unwrap();
            request.body(body)
        } else {
            request
        };

        let request = if let Some(query) = query {
            request.query(query)
        } else {
            request
        };

        let response = self.append_headers(request).send()?;

        let response_status = response.status();

        let json_data = response.bytes()?;
        let json_str = std::str::from_utf8(&json_data).map_err(|_| Error::ResponseWasNotString)?;

        log::debug!("JSON response: {json_str}");

        if response_status != StatusCode::OK {
            // TODO: get useful error message from status message line.
            // perhaps it is in the extensions? Proxmox sometimes returns
            // HTTP 500 Disk is locked, and we need to be able to extract
            // that information, somehow... Ureq client can do this
            return Err(Error::UnknownFailure(response_status));
        }

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
}

#[derive(Debug, Deserialize)]
pub struct Response<T> {
    pub data: Option<T>,
    pub errors: Option<serde_json::Value>,
}
