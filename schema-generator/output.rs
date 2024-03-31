pub struct ClusterClient {
    client: ::std::sync::Arc<::proxmox_api::client::Client>,
    path: String,
}
impl ClusterClient {
    pub fn new(client: ::std::sync::Arc<::proxmox_api::client::Client>) -> Self {
        Self {
            client,
            path: "cluster".to_string(),
        }
    }
}
impl ClusterClient {
    pub fn get(&self) -> Result<Vec<()>, ::proxmox_api::client::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
pub mod nextid {
    pub struct NextidClient {
        client: ::std::sync::Arc<::proxmox_api::client::Client>,
        path: String,
    }
    impl NextidClient {
        pub fn new(
            client: ::std::sync::Arc<::proxmox_api::client::Client>,
            parent_path: &str,
        ) -> Self {
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
        pub fn get(&self, params: GetParams) -> Result<u32, ::proxmox_api::client::Error> {
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
pub mod backup {
    pub struct BackupClient {
        client: ::std::sync::Arc<::proxmox_api::client::Client>,
        path: String,
    }
    impl BackupClient {
        pub fn new(
            client: ::std::sync::Arc<::proxmox_api::client::Client>,
            parent_path: &str,
        ) -> Self {
            Self {
                client,
                path: format!("{}/{}", parent_path, "backup"),
            }
        }
    }
    impl BackupClient {
        pub fn get(&self) -> Result<Vec<()>, ::proxmox_api::client::Error> {
            let path = self.path.to_string();
            self.client.get(&path, &())
        }
    }
    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize, Default)]
    pub struct PostParams {
        pub mailnotification: Option<String>,
        pub protected: Option<bool>,
        pub id: Option<String>,
        pub script: Option<String>,
        pub starttime: Option<String>,
        pub enabled: Option<bool>,
        pub tmpdir: Option<String>,
        pub compress: Option<String>,
        pub stop: Option<bool>,
        pub repeat_missed: Option<bool>,
        pub all: Option<bool>,
        pub node: Option<String>,
        pub mode: Option<String>,
        pub pigz: Option<u32>,
        pub remove: Option<bool>,
        pub ionice: Option<u32>,
        pub schedule: Option<String>,
        pub prune_backups: Option<String>,
        pub performance: Option<String>,
        pub vmid: Option<String>,
        pub lockwait: Option<u32>,
        pub quiet: Option<bool>,
        pub exclude: Option<String>,
        pub maxfiles: Option<u32>,
        pub comment: Option<String>,
        pub bwlimit: Option<u32>,
        pub mailto: Option<String>,
        pub dumpdir: Option<String>,
        pub exclude_path: Option<String>,
        pub stopwait: Option<u32>,
        pub notes_template: Option<String>,
        pub pool: Option<String>,
        pub storage: Option<String>,
        pub zstd: Option<u32>,
        pub dow: Option<String>,
        pub stdexcludes: Option<bool>,
    }
    impl BackupClient {
        pub fn post(&self, params: PostParams) -> Result<(), ::proxmox_api::client::Error> {
            let path = self.path.to_string();
            self.client.post(&path, &params)
        }
    }
    pub mod id {
        pub struct IdClient {
            client: ::std::sync::Arc<::proxmox_api::client::Client>,
            path: String,
        }
        impl IdClient {
            pub fn new(
                client: ::std::sync::Arc<::proxmox_api::client::Client>,
                parent_path: &str,
            ) -> Self {
                Self {
                    client,
                    path: format!("{}/{}", parent_path, "id"),
                }
            }
        }
        impl IdClient {
            pub fn delete(&self) -> Result<(), ::proxmox_api::client::Error> {
                let path = self.path.to_string();
                self.client.delete(&path, &())
            }
        }
        impl IdClient {
            pub fn get(&self) -> Result<(), ::proxmox_api::client::Error> {
                let path = self.path.to_string();
                self.client.get(&path, &())
            }
        }
        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
        pub struct PutParams {
            pub ionice: Option<u32>,
            pub storage: Option<String>,
            pub exclude_path: Option<String>,
            pub zstd: Option<u32>,
            pub quiet: Option<bool>,
            pub exclude: Option<String>,
            pub comment: Option<String>,
            pub dumpdir: Option<String>,
            pub all: Option<bool>,
            pub stopwait: Option<u32>,
            pub repeat_missed: Option<bool>,
            pub tmpdir: Option<String>,
            pub maxfiles: Option<u32>,
            pub mailnotification: Option<String>,
            pub script: Option<String>,
            pub stdexcludes: Option<bool>,
            pub performance: Option<String>,
            pub compress: Option<String>,
            pub vmid: Option<String>,
            pub pigz: Option<u32>,
            pub prune_backups: Option<String>,
            pub starttime: Option<String>,
            pub dow: Option<String>,
            pub lockwait: Option<u32>,
            pub notes_template: Option<String>,
            pub mode: Option<String>,
            pub remove: Option<bool>,
            pub stop: Option<bool>,
            pub pool: Option<String>,
            pub bwlimit: Option<u32>,
            pub delete: Option<String>,
            pub enabled: Option<bool>,
            pub mailto: Option<String>,
            pub node: Option<String>,
            pub schedule: Option<String>,
            pub protected: Option<bool>,
        }
        impl IdClient {
            pub fn put(&self, params: PutParams) -> Result<(), ::proxmox_api::client::Error> {
                let path = self.path.to_string();
                self.client.put(&path, &params)
            }
        }
    }
    impl BackupClient {
        pub fn id(&self, id: &str) -> id::IdClient {
            id::IdClient::new(self.client.clone(), &self.path)
        }
    }
}
impl ClusterClient {
    pub fn backup(&self) -> backup::BackupClient {
        backup::BackupClient::new(self.client.clone(), &self.path)
    }
}
