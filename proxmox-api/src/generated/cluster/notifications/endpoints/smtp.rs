pub mod name;
pub struct SmtpClient<T> {
    client: T,
    path: String,
}
impl<T> SmtpClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/smtp"),
        }
    }
}
impl<T> SmtpClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Returns a list of all smtp endpoints"]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> SmtpClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create a new smtp endpoint"]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl GetOutputItems {
    pub fn new(from_address: String, name: String, origin: Origin, server: String) -> Self {
        Self {
            from_address,
            name,
            origin,
            server,
            author: Default::default(),
            comment: Default::default(),
            disable: Default::default(),
            mailto: Default::default(),
            mailto_user: Default::default(),
            mode: Default::default(),
            port: Default::default(),
            username: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Author of the mail. Defaults to 'Proxmox VE'."]
    #[doc = ""]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Comment"]
    #[doc = ""]
    pub comment: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Disable this target"]
    #[doc = ""]
    pub disable: Option<bool>,
    #[serde(rename = "from-address")]
    #[doc = "`From` address for the mail"]
    #[doc = ""]
    pub from_address: String,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "List of email recipients"]
    #[doc = ""]
    pub mailto: Vec<String>,
    #[serde(rename = "mailto-user")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "List of users"]
    #[doc = ""]
    pub mailto_user: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Determine which encryption method shall be used for the connection."]
    #[doc = ""]
    pub mode: Option<Mode>,
    #[doc = "The name of the endpoint."]
    #[doc = ""]
    pub name: String,
    #[doc = "Show if this entry was created by a user or was built-in"]
    #[doc = ""]
    pub origin: Origin,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The port to be used. Defaults to 465 for TLS based connections, 587 for STARTTLS based connections and port 25 for insecure plain-text connections."]
    #[doc = ""]
    pub port: Option<u64>,
    #[doc = "The address of the SMTP server."]
    #[doc = ""]
    pub server: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Username for SMTP authentication"]
    #[doc = ""]
    pub username: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(from_address: String, name: String, server: String) -> Self {
        Self {
            from_address,
            name,
            server,
            author: Default::default(),
            comment: Default::default(),
            disable: Default::default(),
            mailto: Default::default(),
            mailto_user: Default::default(),
            mode: Default::default(),
            password: Default::default(),
            port: Default::default(),
            username: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Author of the mail. Defaults to 'Proxmox VE'."]
    #[doc = ""]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Comment"]
    #[doc = ""]
    pub comment: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Disable this target"]
    #[doc = ""]
    pub disable: Option<bool>,
    #[serde(rename = "from-address")]
    #[doc = "`From` address for the mail"]
    #[doc = ""]
    pub from_address: String,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "List of email recipients"]
    #[doc = ""]
    pub mailto: Vec<String>,
    #[serde(rename = "mailto-user")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "List of users"]
    #[doc = ""]
    pub mailto_user: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Determine which encryption method shall be used for the connection."]
    #[doc = ""]
    pub mode: Option<Mode>,
    #[doc = "The name of the endpoint."]
    #[doc = ""]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Password for SMTP authentication"]
    #[doc = ""]
    pub password: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The port to be used. Defaults to 465 for TLS based connections, 587 for STARTTLS based connections and port 25 for insecure plain-text connections."]
    #[doc = ""]
    pub port: Option<u64>,
    #[doc = "The address of the SMTP server."]
    #[doc = ""]
    pub server: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Username for SMTP authentication"]
    #[doc = ""]
    pub username: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Determine which encryption method shall be used for the connection."]
#[doc = ""]
pub enum Mode {
    #[serde(rename = "insecure")]
    Insecure,
    #[serde(rename = "starttls")]
    Starttls,
    #[serde(rename = "tls")]
    Tls,
}
impl TryFrom<&str> for Mode {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "insecure" => Ok(Self::Insecure),
            "starttls" => Ok(Self::Starttls),
            "tls" => Ok(Self::Tls),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
impl Default for Mode {
    fn default() -> Self {
        Self::Tls
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Show if this entry was created by a user or was built-in"]
#[doc = ""]
pub enum Origin {
    #[serde(rename = "builtin")]
    Builtin,
    #[serde(rename = "modified-builtin")]
    ModifiedBuiltin,
    #[serde(rename = "user-created")]
    UserCreated,
}
impl TryFrom<&str> for Origin {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "builtin" => Ok(Self::Builtin),
            "modified-builtin" => Ok(Self::ModifiedBuiltin),
            "user-created" => Ok(Self::UserCreated),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
impl<T> SmtpClient<T>
where
    T: crate::client::Client,
{
    pub fn name(&self, name: &str) -> name::NameClient<T> {
        name::NameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
