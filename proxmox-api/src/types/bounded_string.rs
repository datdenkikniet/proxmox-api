use std::{
    hash::{Hash, Hasher},
    sync::{LazyLock, RwLock},
};

static REGEX_CACHE: LazyLock<RwLock<std::collections::HashMap<u64, regex::Regex>>> =
    LazyLock::new(|| RwLock::new(std::collections::HashMap::new()));

fn get_or_compile_regex(pattern: &str) -> Result<regex::Regex, BoundedStringError> {
    let mut h = std::hash::DefaultHasher::new();
    {
        let cache = REGEX_CACHE.read().unwrap();
        pattern.hash(&mut h);
        if let Some(regex) = cache.get(&h.finish()) {
            return Ok(regex.clone());
        }
    }
    let regex = regex::Regex::new(pattern).map_err(BoundedStringError::RegexCreationError)?;
    let result = regex.clone();
    pattern.hash(&mut h);
    REGEX_CACHE.write().unwrap().insert(h.finish(), regex);
    Ok(result)
}

#[derive(Debug, Clone, PartialEq)]
pub enum BoundedStringError {
    TooLong,
    TooShort,
    PatternMismatch,
    RegexCreationError(regex::Error),
}

impl std::fmt::Display for BoundedStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BoundedStringError::TooLong => write!(f, "Given string is too long"),
            BoundedStringError::TooShort => write!(f, "Given string is too short"),
            BoundedStringError::PatternMismatch => write!(f, "Given string does not match pattern"),
            BoundedStringError::RegexCreationError(err) => {
                write!(f, "Regex creation error: {}", err)
            }
        }
    }
}

impl std::error::Error for BoundedStringError {}

pub trait BoundedString {
    const MIN_LENGTH: Option<usize> = None;
    const MAX_LENGTH: Option<usize> = None;
    const DEFAULT: Option<&'static str> = None;
    const PATTERN: Option<&'static str> = None;
    const TYPE_DESCRIPTION: &'static str;

    fn get_value(&self) -> &str;

    fn new(value: String) -> Result<Self, BoundedStringError>
    where
        Self: Sized;

    fn validate(value: &str) -> Result<(), BoundedStringError> {
        if let Some(min_len) = Self::MIN_LENGTH
            && value.len() < min_len
        {
            return Err(BoundedStringError::TooShort);
        } else if let Some(max) = Self::MAX_LENGTH
            && value.len() > max
        {
            return Err(BoundedStringError::TooLong);
        } else if let Some(pattern) = Self::PATTERN {
            let regex = get_or_compile_regex(pattern)?;
            if !regex.is_match(value) {
                return Err(BoundedStringError::PatternMismatch);
            }
        }
        Ok(())
    }
}

use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize_bounded_string<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: BoundedString,
{
    serializer.serialize_str(value.get_value())
}

pub fn deserialize_bounded_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: BoundedString + TryFrom<String, Error = BoundedStringError>,
{
    let value = String::deserialize(deserializer)?;
    T::try_from(value).map_err(|e| {
        serde::de::Error::custom(format!(
            "could not parse as {} with error: {}",
            T::TYPE_DESCRIPTION,
            e
        ))
    })
}
