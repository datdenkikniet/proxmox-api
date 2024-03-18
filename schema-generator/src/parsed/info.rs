use super::{Parameters, Returns};

#[derive(Debug, Clone)]
pub struct Info {
    pub allow_token: bool,
    pub method: Method,
    pub description: Option<String>,
    pub parameters: Parameters,
    pub permissions: (),
    pub returns: Option<Returns>,
    pub protected: bool,
    pub proxy_to: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}

impl TryFrom<&str> for Method {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = match value {
            "GET" => Self::Get,
            "POST" => Self::Post,
            "PUT" => Self::Put,
            "DELETE" => Self::Delete,
            _ => return Err(()),
        };

        Ok(value)
    }
}
