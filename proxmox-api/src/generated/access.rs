pub mod acl;
pub mod domains;
pub mod groups;
pub mod openid;
pub mod password;
pub mod permissions;
pub mod roles;
pub mod tfa;
pub mod ticket;
pub mod users;
pub struct AccessClient<T> {
    client: T,
    path: String,
}
impl<T> AccessClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T) -> Self {
        Self {
            client,
            path: "/access".to_string(),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a AccessClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> AccessClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Directory index."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<(), Vec<GetOutputItems>, T::Error>
    for &AccessClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Get;
    fn exec(&self, params: ()) -> Result<Vec<GetOutputItems>, T::Error> {
        self.get()
    }
}
impl GetOutputItems {
    pub fn new(subdir: String) -> Self {
        Self {
            subdir,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    pub subdir: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> AccessClient<T>
where
    T: crate::client::Client,
{
    pub fn users(&self) -> users::UsersClient<T> {
        users::UsersClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AccessClient<T>
where
    T: crate::client::Client,
{
    pub fn groups(&self) -> groups::GroupsClient<T> {
        groups::GroupsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AccessClient<T>
where
    T: crate::client::Client,
{
    pub fn roles(&self) -> roles::RolesClient<T> {
        roles::RolesClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AccessClient<T>
where
    T: crate::client::Client,
{
    pub fn acl(&self) -> acl::AclClient<T> {
        acl::AclClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AccessClient<T>
where
    T: crate::client::Client,
{
    pub fn domains(&self) -> domains::DomainsClient<T> {
        domains::DomainsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AccessClient<T>
where
    T: crate::client::Client,
{
    pub fn openid(&self) -> openid::OpenidClient<T> {
        openid::OpenidClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AccessClient<T>
where
    T: crate::client::Client,
{
    pub fn tfa(&self) -> tfa::TfaClient<T> {
        tfa::TfaClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AccessClient<T>
where
    T: crate::client::Client,
{
    pub fn ticket(&self) -> ticket::TicketClient<T> {
        ticket::TicketClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AccessClient<T>
where
    T: crate::client::Client,
{
    pub fn password(&self) -> password::PasswordClient<T> {
        password::PasswordClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AccessClient<T>
where
    T: crate::client::Client,
{
    pub fn permissions(&self) -> permissions::PermissionsClient<T> {
        permissions::PermissionsClient::<T>::new(self.client.clone(), &self.path)
    }
}
