pub struct ClusterClient {
    client: ::std::sync::Arc<crate::client::Client>,
    path: String,
}
impl ClusterClient {
    pub fn new(client: ::std::sync::Arc<crate::client::Client>) -> Self {
        Self {
            client,
            path: "cluster".to_string(),
        }
    }
}
impl ClusterClient {
    pub fn get(&self) -> Result<Vec<()>, crate::client::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
pub mod nextid {
    pub struct NextidClient {
        client: ::std::sync::Arc<crate::client::Client>,
        path: String,
    }
    impl NextidClient {
        pub fn new(client: ::std::sync::Arc<crate::client::Client>, parent_path: &str) -> Self {
            Self {
                client,
                path: format!("{}/{}", parent_path, "nextid"),
            }
        }
    }
    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize, Default)]
    pub struct GetParams {
        pub vmid: Option<u32>,
    }
    impl NextidClient {
        pub fn get(&self, params: GetParams) -> Result<u32, crate::client::Error> {
            let path = self.path.to_string();
            self.client.get(&path, &params)
        }
    }
}
impl ClusterClient {
    pub fn nextid(&self) -> nextid::NextidClient {
        nextid::NextidClient::new(self.client.clone(), &self.path)
    }
}
