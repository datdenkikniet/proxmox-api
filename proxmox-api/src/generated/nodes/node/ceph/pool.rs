pub mod name;
pub struct PoolClient<T> {
    client: T,
    path: String,
}
impl<T> PoolClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/pool"),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Application {
    #[serde(rename = "cephfs")]
    Cephfs,
    #[serde(rename = "rbd")]
    Rbd,
    #[serde(rename = "rgw")]
    Rgw,
}
impl Default for Application {
    fn default() -> Self {
        Self::Rbd
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum PgAutoscaleMode {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "on")]
    On,
    #[serde(rename = "warn")]
    Warn,
}
impl Default for PgAutoscaleMode {
    fn default() -> Self {
        Self::Warn
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Type {
    #[serde(rename = "erasure")]
    Erasure,
    #[serde(rename = "replicated")]
    Replicated,
    #[serde(rename = "unknown")]
    Unknown,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct ApplicationMetadataGetOutputItemsApplicationMetadata {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct AutoscaleStatusGetOutputItemsAutoscaleStatus {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetOutputItems {
    pub fn new(
        bytes_used: u64,
        crush_rule: u64,
        crush_rule_name: String,
        min_size: u64,
        percent_used: f64,
        pg_num: u64,
        pool: u64,
        pool_name: String,
        size: u64,
        ty: Type,
    ) -> Self {
        Self {
            bytes_used,
            crush_rule,
            crush_rule_name,
            min_size,
            percent_used,
            pg_num,
            pool,
            pool_name,
            size,
            ty,
            application_metadata: Default::default(),
            autoscale_status: Default::default(),
            pg_autoscale_mode: Default::default(),
            pg_num_final: Default::default(),
            pg_num_min: Default::default(),
            target_size: Default::default(),
            target_size_ratio: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub application_metadata: Option<ApplicationMetadataGetOutputItemsApplicationMetadata>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub autoscale_status: Option<AutoscaleStatusGetOutputItemsAutoscaleStatus>,
    #[serde(
        serialize_with = "crate::serialize_int",
        deserialize_with = "crate::deserialize_int"
    )]
    pub bytes_used: u64,
    #[serde(
        serialize_with = "crate::serialize_int",
        deserialize_with = "crate::deserialize_int"
    )]
    pub crush_rule: u64,
    pub crush_rule_name: String,
    #[serde(
        serialize_with = "crate::serialize_int",
        deserialize_with = "crate::deserialize_int"
    )]
    pub min_size: u64,
    #[serde(
        serialize_with = "crate::serialize_number",
        deserialize_with = "crate::deserialize_number"
    )]
    pub percent_used: f64,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pg_autoscale_mode: Option<String>,
    #[serde(
        serialize_with = "crate::serialize_int",
        deserialize_with = "crate::deserialize_int"
    )]
    pub pg_num: u64,
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pg_num_final: Option<u64>,
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pg_num_min: Option<u64>,
    #[serde(
        serialize_with = "crate::serialize_int",
        deserialize_with = "crate::deserialize_int"
    )]
    pub pool: u64,
    pub pool_name: String,
    #[serde(
        serialize_with = "crate::serialize_int",
        deserialize_with = "crate::deserialize_int"
    )]
    pub size: u64,
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub target_size: Option<u64>,
    #[serde(
        serialize_with = "crate::serialize_number_optional",
        deserialize_with = "crate::deserialize_number_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub target_size_ratio: Option<f64>,
    #[serde(rename = "type")]
    pub ty: Type,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> PoolClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List all pools and their settings (which are settable by the POST/PUT endpoints)."]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl PostParams {
    pub fn new(name: String) -> Self {
        Self {
            name,
            add_storages: Default::default(),
            application: Default::default(),
            crush_rule: Default::default(),
            erasure_coding: Default::default(),
            min_size: Default::default(),
            pg_autoscale_mode: Default::default(),
            pg_num: Default::default(),
            pg_num_min: Default::default(),
            size: Default::default(),
            target_size: Default::default(),
            target_size_ratio: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Configure VM and CT storage using the new pool."]
    pub add_storages: Option<()>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The application of the pool."]
    pub application: Option<Application>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The rule to use for mapping object placement in the cluster."]
    pub crush_rule: Option<String>,
    #[serde(rename = "erasure-coding")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Create an erasure coded pool for RBD with an accompaning replicated pool for metadata storage. With EC, the common ceph options 'size', 'min_size' and 'crush_rule' parameters will be applied to the metadata pool."]
    pub erasure_coding: Option<String>,
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Minimum number of replicas per object"]
    pub min_size: Option<u64>,
    #[doc = "The name of the pool. It must be unique."]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The automatic PG scaling mode of the pool."]
    pub pg_autoscale_mode: Option<PgAutoscaleMode>,
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Number of placement groups."]
    pub pg_num: Option<u64>,
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Minimal number of placement groups."]
    pub pg_num_min: Option<u64>,
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Number of replicas per object"]
    pub size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The estimated target size of the pool for the PG autoscaler."]
    pub target_size: Option<String>,
    #[serde(
        serialize_with = "crate::serialize_number_optional",
        deserialize_with = "crate::deserialize_number_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The estimated target ratio of the pool for the PG autoscaler."]
    pub target_size_ratio: Option<f64>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> PoolClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create Ceph pool"]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl<T> PoolClient<T>
where
    T: crate::client::Client,
{
    pub fn name(&self, name: &str) -> name::NameClient<T> {
        name::NameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
