use std::borrow::Cow;

use crate::{
    parsed::{info::Method, Type},
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

        let returns = if let Some(returns) = value.returns.as_ref() {
            Some(Self::parse_returns(returns)?)
        } else {
            None
        };

        println!("{returns:?}");

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
        // if let Some(ty) = &returns.ty {
        //     let ty = Type::parse(
        //         &returns.optional,
        //         &ty,
        //         None,
        //         None,
        //         returns.items.as_ref(),
        //         returns.format.as_ref(),
        //         returns.properties.as_ref(),
        //         returns.additional_properties.map(|v| v != 0),
        //     )?;

        //     Ok(Returns {
        //         ty,
        //         description: returns.description.as_ref().map(Cow::to_string),
        //     })
        // } else {
        //     Err(format!("Meh"))
        // }
        todo!()
    }

    fn parse_parameters(parameters: &raw::Parameters) -> Result<Parameters, String> {
        Ok(Parameters {
            allow_additional_properties: false,
            ty: Type {
                kind: super::r#type::TypeKind::Boolean,
                optional: false,
            },
        })
    }

    fn parse_property(name: &str, prop: &raw::Property) {
        let prop: Result<Property, _> = Property::try_from((name, prop));

        match prop {
            Ok(prop) => println!("{prop:?}"),
            Err(msg) => eprintln!("{msg}"),
        }
    }
}
