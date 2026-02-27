pub mod client;

mod path;
pub use path::{Path, PathElement};

pub mod types;

mod generated;
#[allow(unused)]
pub use generated::*;

#[cfg(any(feature = "reqwest-client", feature = "ureq-client"))]
mod clients;

#[cfg(any(feature = "reqwest-client", feature = "ureq-client"))]
pub use clients::*;
