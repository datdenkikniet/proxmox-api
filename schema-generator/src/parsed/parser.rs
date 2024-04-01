use std::borrow::Cow;

use crate::{
    parsed::info::Method,
    raw::{self, flattened::Collection},
};

use super::{Info, Parameters, Property, Returns};

pub struct Parser;

impl Parser {
    pub fn parse(schema: &Collection) {
        schema.iter().for_each(|v| {
            Self::parse_value(v);
        });
    }

    fn parse_value(value: &raw::flattened::Value) -> Result<(), String> {
        for (name, v) in value.info.iter() {
            let info = Self::parse_info(name, v)?;
        }

        Ok(())
    }

    fn parse_info(name: &str, value: &raw::Info) -> Result<Info, String> {
        assert_eq!(name, value.method);

        let allow_token = value.allow_token.map(|v| v == 1).unwrap_or(true);
        let method = Method::try_from(value.method.as_ref())
            .map_err(|_| format!("Unknown method {}", value.method))?;
        let description = value.description.as_ref().map(Cow::to_string);
        let parameters = Self::parse_parameters(&value.parameters)?;
        let returns = value
            .returns
            .as_ref()
            .map(|r| Self::parse_returns(r))
            .transpose()?;
        let protected = value.protected.map(|v| v == 1).unwrap_or(false);
        let proxy_to = value.proxy_to.as_ref().map(Cow::to_string);

        Ok(Info {
            allow_token,
            method,
            description,
            permissions: (),
            parameters,
            returns,
            protected,
            proxy_to,
        })
    }

    fn parse_returns(returns: &raw::Returns) -> Result<Returns, String> {
        for (name, prop) in returns.properties.iter() {
            Self::parse_property(&name, prop);
        }

        todo!()
    }

    fn parse_parameters(parameters: &raw::Parameters) -> Result<Parameters, String> {
        for (name, prop) in parameters.properties.iter() {
            Self::parse_property(&name, prop);
        }

        Err("bruh".into())
    }

    fn parse_property(name: &str, prop: &raw::Property) {
        let prop: Result<Property, _> = (name, prop).try_into();

        match prop {
            Ok(prop) => println!("{prop:?}"),
            Err(msg) => eprintln!("{msg}"),
        }
    }
}
