use std::sync::Arc;

use rustls::{
    client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier},
    SignatureScheme,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use ureq::Request;

use super::base_access::{AuthState, Ticket, TicketResponse};

#[derive(Debug)]
pub enum Error {
    Ureq(ureq::Error),
    EncounteredErrors(serde_json::Value),
    ResponseWasNotString,
    DecodingFailed(String, serde_json::Error),
    Other(&'static str),
    UnknownFailure,
}

impl From<ureq::Error> for Error {
    fn from(value: ureq::Error) -> Self {
        Self::Ureq(value)
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    host: String,

    user: String,
    realm: String,

    auth_state: AuthState,
}

impl Client {
    fn new_empty(host: &str, user: &str, realm: &str) -> Self {
        Self {
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

    fn append_headers(&self, request: Request) -> Request {
        let auth_state = &self.auth_state;

        let request = if let Some(auth_ticket) = auth_state.ticket() {
            request.set("Cookie", &format!("PVEAuthCookie={auth_ticket}"))
        } else {
            request
        };

        let request = if let Some(csrf) = auth_state.csrf() {
            request.set("CSRFPreventionToken", csrf.as_str())
        } else {
            request
        };

        let request = if let Some(api_token) = auth_state.api_token() {
            request.set("Authorization", &format!("PVEAPIToken={api_token}"))
        } else {
            request
        };

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
        let uri = self.route(path.as_ref());
        let uri = uri.as_str();

        #[derive(Debug)]
        struct Verifier;

        impl ServerCertVerifier for Verifier {
            fn verify_server_cert(
                &self,
                _: &rustls::pki_types::CertificateDer<'_>,
                _: &[rustls::pki_types::CertificateDer<'_>],
                _: &rustls::pki_types::ServerName<'_>,
                _: &[u8],
                _: rustls::pki_types::UnixTime,
            ) -> Result<ServerCertVerified, rustls::Error> {
                Ok(ServerCertVerified::assertion())
            }

            fn verify_tls12_signature(
                &self,
                _: &[u8],
                _: &rustls::pki_types::CertificateDer<'_>,
                _: &rustls::DigitallySignedStruct,
            ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error>
            {
                Ok(HandshakeSignatureValid::assertion())
            }

            fn verify_tls13_signature(
                &self,
                _: &[u8],
                _: &rustls::pki_types::CertificateDer<'_>,
                _: &rustls::DigitallySignedStruct,
            ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error>
            {
                Ok(HandshakeSignatureValid::assertion())
            }

            fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
                [
                    SignatureScheme::ECDSA_NISTP256_SHA256,
                    SignatureScheme::ECDSA_NISTP384_SHA384,
                    SignatureScheme::ECDSA_NISTP521_SHA512,
                    SignatureScheme::ED25519,
                    SignatureScheme::ED448,
                    SignatureScheme::RSA_PKCS1_SHA1,
                    SignatureScheme::RSA_PKCS1_SHA256,
                    SignatureScheme::RSA_PKCS1_SHA384,
                    SignatureScheme::RSA_PKCS1_SHA512,
                    SignatureScheme::RSA_PSS_SHA256,
                    SignatureScheme::RSA_PSS_SHA384,
                    SignatureScheme::RSA_PSS_SHA512,
                ]
                .to_vec()
            }
        }

        let config = rustls::ClientConfig::builder()
            .dangerous()
            .with_custom_certificate_verifier(Arc::new(Verifier))
            .with_no_client_auth();

        let agent = ureq::AgentBuilder::new()
            .tls_config(Arc::new(config))
            .build();

        let request = match method {
            crate::client::Method::Post => agent.post(uri),
            crate::client::Method::Get => agent.get(uri),
            crate::client::Method::Put => agent.put(uri),
            crate::client::Method::Delete => agent.delete(uri),
        };

        let request = if let Some(query) = query {
            let url = request.request_url().unwrap();
            let mut url = url.as_url().clone();
            let mut pairs = url.query_pairs_mut();
            let serializer = serde_urlencoded::Serializer::new(&mut pairs);
            query.serialize(serializer).unwrap();

            let final_url = pairs.finish();

            let pairs = final_url.query_pairs();

            let mut request = request;
            for (k, v) in pairs {
                request = request.query(&k, &v);
            }

            request
        } else {
            request
        };

        log::debug!("{} {}", method, request.url());

        let request = self.append_headers(request);

        let response = if let Some(body) = body {
            let body = serde_urlencoded::to_string(body).unwrap();
            request.send_string(&body)
        } else {
            request.call()
        };

        let response = response?;

        let json_str = response.into_string().unwrap();

        log::debug!("JSON response: {json_str}");

        let result: Response<R> = serde_json::from_str(&json_str)
            .map_err(|e| Error::DecodingFailed(json_str.into(), e))?;

        if let Some(data) = result.data {
            Ok(data)
        } else if let Some(errors) = result.errors {
            Err(Error::EncounteredErrors(errors))
        } else {
            Err(Error::UnknownFailure)
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Response<T> {
    pub data: Option<T>,
    pub errors: Option<serde_json::Value>,
}
