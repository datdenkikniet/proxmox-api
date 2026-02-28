#[derive(Debug, Clone)]
pub struct ApiversionClient<T> {
    client: T,
    path: String,
}
impl<T> ApiversionClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/apiversion"),
        }
    }
}
impl<T> ApiversionClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Return the version of the cluster join API available on this node."]
    #[doc = ""]
    pub fn get(&self) -> Result<Int, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Int(i128);
impl crate::types::bounded_integer::BoundedInteger for Int {
    const MIN: Option<i128> = Some(0i128);
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = None::<i128>;
    const TYPE_DESCRIPTION: &'static str = "an integer greater than or equal to 0";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for Int {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for Int {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for Int {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
