pub struct ScheduleNowClient<T> {
    client: T,
    path: String,
}
impl<T> ScheduleNowClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/schedule_now"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a ScheduleNowClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> ScheduleNowClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Schedule replication job to start as soon as possible."]
    #[doc = ""]
    pub fn post(&self) -> Result<String, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.post(&path, &())
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<(), String, T::Error> for &ScheduleNowClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Post;
    fn exec(&self, params: ()) -> Result<String, T::Error> {
        self.post()
    }
}
