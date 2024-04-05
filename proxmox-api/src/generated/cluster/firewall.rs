pub mod aliases;
pub mod groups;
pub mod ipset;
pub mod macros;
pub mod options;
pub mod refs;
pub mod rules;
pub struct FirewallClient<T> {
    client: T,
    path: String,
}
impl<T> FirewallClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/firewall"),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutputItems {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> FirewallClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Directory index."]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> FirewallClient<T>
where
    T: crate::client::Client,
{
    pub fn groups(&self) -> groups::GroupsClient<T> {
        groups::GroupsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> FirewallClient<T>
where
    T: crate::client::Client,
{
    pub fn rules(&self) -> rules::RulesClient<T> {
        rules::RulesClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> FirewallClient<T>
where
    T: crate::client::Client,
{
    pub fn ipset(&self) -> ipset::IpsetClient<T> {
        ipset::IpsetClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> FirewallClient<T>
where
    T: crate::client::Client,
{
    pub fn aliases(&self) -> aliases::AliasesClient<T> {
        aliases::AliasesClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> FirewallClient<T>
where
    T: crate::client::Client,
{
    pub fn options(&self) -> options::OptionsClient<T> {
        options::OptionsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> FirewallClient<T>
where
    T: crate::client::Client,
{
    pub fn macros(&self) -> macros::MacrosClient<T> {
        macros::MacrosClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> FirewallClient<T>
where
    T: crate::client::Client,
{
    pub fn refs(&self) -> refs::RefsClient<T> {
        refs::RefsClient::<T>::new(self.client.clone(), &self.path)
    }
}
