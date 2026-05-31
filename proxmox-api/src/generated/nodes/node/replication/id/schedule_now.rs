#[derive(Debug, Clone)]
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
impl<T> ScheduleNowClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Schedule replication job to start as soon as possible."]
    #[doc = ""]
    #[doc = "Requires the VM.Replicate permission on /vms/\\<vmid\\>."]
    pub async fn post(&self) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &()).await
    }
}
