use super::Type;

#[derive(Debug, Clone)]
pub struct Parameters {
    pub allow_additional_properties: bool,
    pub ty: Type,
}
