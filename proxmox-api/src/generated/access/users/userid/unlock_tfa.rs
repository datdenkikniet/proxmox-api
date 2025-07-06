#[derive(Debug, Clone)]
pub struct UnlockTfaClient<T> {
    client: T,
    path: String,
}
impl<T> UnlockTfaClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/unlock-tfa"),
        }
    }
}
impl<T> UnlockTfaClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Unlock a user's TFA authentication."]
    #[doc = ""]
    pub fn put(&self) -> Result<bool, T::Error> {
        let path = self.path.to_string();
        Ok(self.client.put::<_, crate::types::Bool>(&path, &())?.get())
    }
}
