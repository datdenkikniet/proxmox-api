pub struct RepositoriesClient<T> {
    client: T,
    path: String,
}
impl<T> RepositoriesClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/repositories"),
        }
    }
}
impl<T> RepositoriesClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get APT repository information."]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> RepositoriesClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Change the properties of a repository. Currently only allows enabling/disabling."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl<T> RepositoriesClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Add a standard repository to the configuration"]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
impl ErrorsGetOutputErrorsItems {
    pub fn new(error: String, path: String) -> Self {
        Self {
            error,
            path,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct ErrorsGetOutputErrorsItems {
    #[doc = "The error message"]
    #[doc = ""]
    pub error: String,
    #[doc = "Path to the problematic file."]
    #[doc = ""]
    pub path: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl FilesGetOutputFilesItems {
    pub fn new(
        digest: Vec<u64>,
        file_type: FileType,
        path: String,
        repositories: Vec<RepositoriesGetOutputFilesItemsRepositoriesItems>,
    ) -> Self {
        Self {
            digest,
            file_type,
            path,
            repositories,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct FilesGetOutputFilesItems {
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "Digest of the file as bytes."]
    #[doc = ""]
    pub digest: Vec<u64>,
    #[serde(rename = "file-type")]
    #[doc = "Format of the file."]
    #[doc = ""]
    pub file_type: FileType,
    #[doc = "Path to the problematic file."]
    #[doc = ""]
    pub path: String,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "The parsed repositories."]
    #[doc = ""]
    pub repositories: Vec<RepositoriesGetOutputFilesItemsRepositoriesItems>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetOutput {
    pub fn new(
        digest: String,
        errors: Vec<ErrorsGetOutputErrorsItems>,
        files: Vec<FilesGetOutputFilesItems>,
        infos: Vec<InfosGetOutputInfosItems>,
        standard_repos: Vec<StandardReposGetOutputStandardReposItems>,
    ) -> Self {
        Self {
            digest,
            errors,
            files,
            infos,
            standard_repos,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[doc = "Common digest of all files."]
    #[doc = ""]
    pub digest: String,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "List of problematic repository files."]
    #[doc = ""]
    pub errors: Vec<ErrorsGetOutputErrorsItems>,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "List of parsed repository files."]
    #[doc = ""]
    pub files: Vec<FilesGetOutputFilesItems>,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "Additional information/warnings for APT repositories."]
    #[doc = ""]
    pub infos: Vec<InfosGetOutputInfosItems>,
    #[serde(rename = "standard-repos")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "List of standard repositories and their configuration status"]
    #[doc = ""]
    pub standard_repos: Vec<StandardReposGetOutputStandardReposItems>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl InfosGetOutputInfosItems {
    pub fn new(index: String, kind: String, message: String, path: String) -> Self {
        Self {
            index,
            kind,
            message,
            path,
            property: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct InfosGetOutputInfosItems {
    #[doc = "Index of the associated repository within the file."]
    #[doc = ""]
    pub index: String,
    #[doc = "Kind of the information (e.g. warning)."]
    #[doc = ""]
    pub kind: String,
    #[doc = "Information message."]
    #[doc = ""]
    pub message: String,
    #[doc = "Path to the associated file."]
    #[doc = ""]
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Property from which the info originates."]
    #[doc = ""]
    pub property: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl OptionsGetOutputFilesItemsRepositoriesItemsOptionsItems {
    pub fn new(key: String, values: Vec<String>) -> Self {
        Self {
            key,
            values,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct OptionsGetOutputFilesItemsRepositoriesItemsOptionsItems {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    pub values: Vec<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(index: u64, path: String) -> Self {
        Self {
            index,
            path,
            digest: Default::default(),
            enabled: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Digest to detect modifications."]
    #[doc = ""]
    pub digest: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Whether the repository should be enabled or not."]
    #[doc = ""]
    pub enabled: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "Index within the file (starting from 0)."]
    #[doc = ""]
    pub index: u64,
    #[doc = "Path to the containing file."]
    #[doc = ""]
    pub path: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PutParams {
    pub fn new(handle: String) -> Self {
        Self {
            handle,
            digest: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PutParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Digest to detect modifications."]
    #[doc = ""]
    pub digest: Option<String>,
    #[doc = "Handle that identifies a repository."]
    #[doc = ""]
    pub handle: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl RepositoriesGetOutputFilesItemsRepositoriesItems {
    pub fn new(
        enabled: bool,
        filetype: FileType,
        suites: Vec<String>,
        types: Vec<Types>,
        uris: Vec<String>,
    ) -> Self {
        Self {
            enabled,
            filetype,
            suites,
            types,
            uris,
            comment: Default::default(),
            components: Default::default(),
            options: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct RepositoriesGetOutputFilesItemsRepositoriesItems {
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Associated comment"]
    #[doc = ""]
    pub comment: Option<String>,
    #[serde(rename = "Components")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "List of repository components"]
    #[doc = ""]
    pub components: Vec<String>,
    #[serde(rename = "Enabled")]
    #[serde(
        serialize_with = "crate::types::serialize_bool",
        deserialize_with = "crate::types::deserialize_bool"
    )]
    #[doc = "Whether the repository is enabled or not"]
    #[doc = ""]
    pub enabled: bool,
    #[serde(rename = "FileType")]
    #[doc = "Format of the defining file."]
    #[doc = ""]
    pub filetype: FileType,
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "Additional options"]
    #[doc = ""]
    pub options: Vec<OptionsGetOutputFilesItemsRepositoriesItemsOptionsItems>,
    #[serde(rename = "Suites")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "List of package distribuitions"]
    #[doc = ""]
    pub suites: Vec<String>,
    #[serde(rename = "Types")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "List of package types."]
    #[doc = ""]
    pub types: Vec<Types>,
    #[serde(rename = "URIs")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "List of repository URIs."]
    #[doc = ""]
    pub uris: Vec<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl StandardReposGetOutputStandardReposItems {
    pub fn new(handle: String, name: String) -> Self {
        Self {
            handle,
            name,
            status: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct StandardReposGetOutputStandardReposItems {
    #[doc = "Handle to identify the repository."]
    #[doc = ""]
    pub handle: String,
    #[doc = "Full name of the repository."]
    #[doc = ""]
    pub name: String,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Indicating enabled/disabled status, if the repository is configured."]
    #[doc = ""]
    pub status: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Format of the file."]
#[doc = ""]
pub enum FileType {
    #[serde(rename = "list")]
    List,
    #[serde(rename = "sources")]
    Sources,
}
impl TryFrom<&str> for FileType {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "list" => Ok(Self::List),
            "sources" => Ok(Self::Sources),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
pub enum Types {
    #[serde(rename = "deb")]
    Deb,
    #[serde(rename = "deb-src")]
    DebSrc,
}
impl TryFrom<&str> for Types {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "deb" => Ok(Self::Deb),
            "deb-src" => Ok(Self::DebSrc),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
