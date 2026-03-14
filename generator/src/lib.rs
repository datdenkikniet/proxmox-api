pub mod raw;

mod path;
pub use path::{Path, PathElement};

mod generator;
pub use generator::{ClientModDef, Generator};

mod name_utils;
use name_utils::{name_to_ident, name_to_underscore_name};
