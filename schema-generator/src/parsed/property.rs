use std::borrow::Cow;

use crate::raw::{self, ParametersOrU32};

use super::Type;

#[derive(Debug, Clone)]
pub struct Property {
    pub name: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub ty_text: Option<String>,
    pub ty: Type,
}

impl TryFrom<(&'_ str, &'_ raw::Property<'_>)> for Property {
    type Error = String;

    fn try_from((name, value): (&str, &raw::Property)) -> Result<Self, Self::Error> {
        let ty = value
            .ty
            .as_ref()
            .ok_or_else(|| format!("{name} does not have a type."))?;

        let ty: Type = Type::parse(
            &value.optional,
            ty.as_ref(),
            value.ty_text.as_ref(),
            value.enum_values.as_ref(),
            value.items.as_ref(),
            value.format.as_ref(),
            value.properties.as_ref(),
            value
                .additional_properties
                .as_ref()
                .map(ParametersOrU32::allow_additional),
        )?;

        let description = value.description.as_ref().map(Cow::to_string);
        let ty_text = value.ty_text.as_ref().map(Cow::to_string);

        Ok(Self {
            name: name.to_string(),
            title: value.title.as_ref().map(|v| v.to_string()),
            ty,
            description,
            ty_text,
        })
    }
}
