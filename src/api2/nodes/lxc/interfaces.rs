use std::{
    marker::PhantomData,
    net::{Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Interfaces(pub Vec<Interface>);

#[derive(Debug, Deserialize)]
pub struct Interface {
    // TODO: MacAddress
    #[serde(rename = "hwaddr")]
    pub hw_addr: String,
    pub name: String,

    pub inet: Option<Ipv4Cidr>,
    pub inet6: Option<Ipv6Cidr>,
}

trait NamedWithMask {
    const NAME: &'static str;
    const MAX_MASK_BITS: u8;
}

struct Visitor<I, T> {
    _phantom: PhantomData<(I, T)>,
}

impl<I, T> Default for Visitor<I, T> {
    fn default() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
}

impl<I, T> serde::de::Visitor<'_> for Visitor<I, T>
where
    I: FromStr,
    T: NamedWithMask + From<(I, u8)>,
{
    type Value = T;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "A validly formatted {} address with bit mask.", T::NAME)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let (address, mask) = v
            .split_once("/")
            .ok_or_else(|| E::custom(format!("Invalid {} CIDR format.", T::NAME)))?;

        let address = I::from_str(address)
            .map_err(|_| E::custom(format!("Invalid {} address {address}", T::NAME)))?;

        let mask = u8::from_str(mask)
            .map_err(|_| E::custom(format!("Invalid {} mask {mask}.", T::NAME)))?;

        if mask > T::MAX_MASK_BITS {
            Err(E::custom(format!(
                "Mask {mask} is out of range for {} CIDR mask.",
                T::NAME
            )))
        } else {
            Ok(T::from((address, mask)))
        }
    }
}

#[derive(Debug)]
pub struct Ipv4Cidr {
    pub address: Ipv4Addr,
    pub mask_bits: u8,
}

impl From<(Ipv4Addr, u8)> for Ipv4Cidr {
    fn from((address, mask_bits): (Ipv4Addr, u8)) -> Self {
        Self { address, mask_bits }
    }
}

impl NamedWithMask for Ipv4Cidr {
    const NAME: &'static str = "IPv4";
    const MAX_MASK_BITS: u8 = 32;
}

impl<'de> Deserialize<'de> for Ipv4Cidr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(Visitor::default())
    }
}

#[derive(Debug)]
pub struct Ipv6Cidr {
    pub address: Ipv6Addr,
    pub mask_bits: u8,
}

impl From<(Ipv6Addr, u8)> for Ipv6Cidr {
    fn from((address, mask_bits): (Ipv6Addr, u8)) -> Self {
        Self { address, mask_bits }
    }
}

impl NamedWithMask for Ipv6Cidr {
    const NAME: &'static str = "IPv6";
    const MAX_MASK_BITS: u8 = 128;
}

impl<'de> Deserialize<'de> for Ipv6Cidr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(Visitor::default())
    }
}
