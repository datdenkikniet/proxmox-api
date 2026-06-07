use reqwest::{Method, RequestBuilder, StatusCode};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use super::base_access::{AuthState, Ticket, TicketResponse};

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    EncounteredErrors(serde_json::Value),
    ResponseWasNotString,
    DecodingFailed(String, serde_json::Error),
    UrlEncodingFailed(String),
    UnknownFailure(StatusCode, Option<String>),
    Other(&'static str),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Reqwest(e) => write!(f, "{e}"),
            Error::EncounteredErrors(v) => write!(f, "Proxmox returned errors: {v}"),
            Error::ResponseWasNotString => write!(f, "response body was not valid UTF-8"),
            Error::DecodingFailed(text, e) => {
                write!(f, "failed to decode response: {e}; body: {text}")
            }
            Error::UrlEncodingFailed(msg) => write!(f, "failed to URL-encode request body: {msg}"),
            Error::UnknownFailure(status, body) => {
                write!(f, "HTTP {status}")?;
                if let Some(body) = body {
                    write!(f, ": {body}")?;
                }
                Ok(())
            }
            Error::Other(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Reqwest(e) => Some(e),
            _ => None,
        }
    }
}

fn extract_message(body: &str) -> String {
    serde_json::from_str::<serde_json::Value>(body)
        .ok()
        .and_then(|v| v.get("message").and_then(|m| m.as_str().map(String::from)))
        .unwrap_or_else(|| body.to_string())
}

async fn parse_response<R: serde::de::DeserializeOwned>(
    response: reqwest::Response,
) -> Result<R, Error> {
    let response_status = response.status();
    let json_data = response.bytes().await?;
    let json_str = std::str::from_utf8(&json_data).map_err(|_| Error::ResponseWasNotString)?;

    log::debug!("JSON response: {json_str}");

    if response_status != StatusCode::OK {
        return Err(Error::UnknownFailure(
            response_status,
            Some(extract_message(json_str)),
        ));
    }

    let result: Response<R> =
        serde_json::from_str(json_str).map_err(|e| Error::DecodingFailed(json_str.into(), e))?;

    if let Some(data) = result.data {
        Ok(data)
    } else if let Some(errors) = result.errors {
        Err(Error::EncounteredErrors(errors))
    } else {
        Err(Error::UnknownFailure(
            response_status,
            Some(extract_message(json_str)),
        ))
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    client: reqwest::Client,
    host: String,

    user: String,
    realm: String,

    auth_state: AuthState,
}

impl Client {
    fn client() -> reqwest::Client {
        reqwest::ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .build()
            .expect("failed to build HTTP client")
    }

    pub fn new(host: &str, user: &str, realm: &str, client: Option<reqwest::Client>) -> Self {
        Self {
            client: client.unwrap_or_else(Self::client),
            host: host.to_string(),
            user: user.into(),
            realm: realm.into(),
            auth_state: AuthState::new(),
        }
    }

    pub fn with_api_token(self, token_id: &str, token: &str) -> Self {
        // PVEAPIToken=USER@REALM!TOKENID=UUID
        self.auth_state
            .set_api_token(&self.user, &self.realm, token_id, token);
        self
    }

    pub async fn with_login(self, password: &str) -> Result<Self, Error> {
        self.login(password).await?;
        Ok(self)
    }

    pub async fn with_ticket(self, ticket: &str, csrf: &str) -> Result<Self, Error> {
        self.auth_state.set_csrf(ticket.into(), csrf.into());
        self.refresh_auth_ticket(true).await?;
        Ok(self)
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

    async fn login(&self, password: &str) -> Result<(), Error> {
        let user = self.user.to_string();
        let realm = self.realm.to_string();
        let request = Ticket::new(&user, &realm, password);

        let csrf_details: TicketResponse =
            crate::client::Client::post(self, "/access/ticket", &request).await?;

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
    pub async fn refresh_auth_ticket(&self, force: bool) -> Result<(), Error> {
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
            self.login(&auth_ticket).await?;
        }

        Ok(())
    }
}

impl crate::client::Client for Client {
    type Error = Error;

    async fn upload<B, R>(&self, path: &str, body: B, data: Vec<u8>) -> Result<R, Error>
    where
        B: IntoIterator<Item = (String, String)> + crate::client::AsFilename,
        R: DeserializeOwned,
    {
        log::debug!("POST (upload) {}", path);

        let filename = body.as_filename();

        let mut form = reqwest::multipart::Form::new();
        for (key, value) in body {
            form = form.text(key, value);
        }

        let file_part = reqwest::multipart::Part::bytes(data)
            .file_name(filename)
            .mime_str("application/octet-stream")
            .expect("known-valid MIME type");
        form = form.part("filename", file_part);

        let request = self.client.request(Method::POST, self.route(path));
        let response = self.append_headers(request.multipart(form)).send().await?;

        parse_response(response).await
    }

    async fn request_with_body_and_query<B, Q, R>(
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
            let body = serde_urlencoded::to_string(body)
                .map_err(|e| Error::UrlEncodingFailed(e.to_string()))?;
            request.body(body)
        } else {
            request
        };

        let request = if let Some(query) = query {
            request.query(query)
        } else {
            request
        };

        let response = self.append_headers(request).send().await?;

        parse_response(response).await
    }
}

#[derive(Debug, Deserialize)]
pub struct Response<T> {
    pub data: Option<T>,
    pub errors: Option<serde_json::Value>,
}
