mod base_access;

#[cfg(feature = "reqwest-client")]
mod reqwest;

#[cfg(feature = "reqwest-client")]
pub use reqwest::{Client as ReqwestClient, Error as ReqwestError};

#[cfg(feature = "ureq-client")]
mod ureq;

#[cfg(feature = "ureq-client")]
pub use ureq::{Client as UreqClient, Error as UreqError};
