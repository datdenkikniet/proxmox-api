use super::Type;

#[derive(Debug, Clone)]
pub struct Returns {
    pub ty: Type,
    pub description: Option<String>,
}
