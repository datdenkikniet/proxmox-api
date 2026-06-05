use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Method {
    Post,
    Get,
    Put,
    Delete,
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::Post => write!(f, "POST"),
            Method::Get => write!(f, "GET"),
            Method::Put => write!(f, "PUT"),
            Method::Delete => write!(f, "DELETE"),
        }
    }
}

pub trait Client: Clone {
    type Error: core::fmt::Debug;

    /// Upload a file to a Proxmox VE API endpoint using multipart/form-data.
    fn upload<B, R>(
        &self,
        path: &str,
        body: &B,
        data: Vec<u8>,
    ) -> impl Future<Output = Result<R, Self::Error>>
    where
        B: Serialize,
        R: DeserializeOwned;

    /// Transmit an authenticated request to a Proxmox VE API endpoint
    /// using the provided method, path, body, and query.
    fn request_with_body_and_query<B, Q, R>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
        query: Option<&Q>,
    ) -> impl Future<Output = Result<R, Self::Error>>
    where
        B: Serialize,
        Q: Serialize,
        R: DeserializeOwned;

    fn request_with_body<B, R>(
        &self,
        method: Method,
        path: &str,
        body: &B,
    ) -> impl Future<Output = Result<R, Self::Error>>
    where
        B: Serialize,
        R: DeserializeOwned,
    {
        self.request_with_body_and_query::<_, (), _>(method, path, Some(body), None)
    }

    fn request_with_query<Q, R>(
        &self,
        method: Method,
        path: &str,
        query: &Q,
    ) -> impl Future<Output = Result<R, Self::Error>>
    where
        Q: Serialize,
        R: DeserializeOwned,
    {
        self.request_with_body_and_query::<(), _, _>(method, path, None, Some(query))
    }

    fn put<B, R>(&self, path: &str, body: &B) -> impl Future<Output = Result<R, Self::Error>>
    where
        B: Serialize,
        R: DeserializeOwned,
    {
        self.request_with_body(Method::Put, path, body)
    }

    fn post<B, R>(&self, path: &str, body: &B) -> impl Future<Output = Result<R, Self::Error>>
    where
        B: Serialize,
        R: DeserializeOwned,
    {
        self.request_with_body(Method::Post, path, body)
    }

    fn delete<B, R>(&self, path: &str, query: &B) -> impl Future<Output = Result<R, Self::Error>>
    where
        B: Serialize,
        R: DeserializeOwned,
    {
        self.request_with_query(Method::Delete, path, query)
    }

    fn get<Q, R>(&self, path: &str, query: &Q) -> impl Future<Output = Result<R, Self::Error>>
    where
        Q: Serialize,
        R: DeserializeOwned,
    {
        self.request_with_query(Method::Get, path, query)
    }
}

impl<T> Client for &T
where
    T: Client,
{
    type Error = <T as Client>::Error;

    fn upload<B, R>(
        &self,
        path: &str,
        body: &B,
        data: Vec<u8>,
    ) -> impl Future<Output = Result<R, Self::Error>>
    where
        B: Serialize,
        R: DeserializeOwned,
    {
        T::upload(self, path, body, data)
    }

    fn request_with_body_and_query<B, Q, R>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
        query: Option<&Q>,
    ) -> impl Future<Output = Result<R, Self::Error>>
    where
        B: Serialize,
        Q: Serialize,
        R: DeserializeOwned,
    {
        T::request_with_body_and_query(self, method, path, body, query)
    }
}
