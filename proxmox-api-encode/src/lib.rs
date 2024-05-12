pub mod de;
pub mod error;
pub mod ser;

pub trait DefaultKey {
    fn default_key() -> String;
}

#[cfg(test)]
mod test;
