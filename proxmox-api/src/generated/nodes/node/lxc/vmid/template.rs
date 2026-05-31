#[derive(Debug, Clone)]
pub struct TemplateClient<T> {
    client: T,
    path: String,
}
impl<T> TemplateClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/template"),
        }
    }
}
impl<T> TemplateClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create a Template."]
    #[doc = ""]
    #[doc = "Permission check: perm(\"/vms/{vmid}\", [\"VM.Allocate\"])"]
    #[doc = "You need 'VM.Allocate' permissions on /vms/{vmid}"]
    pub async fn post(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &()).await
    }
}
