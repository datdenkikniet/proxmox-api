mod bool;
pub use bool::{
    Bool, deserialize_bool, deserialize_bool_optional, serialize_bool, serialize_bool_optional,
};

mod integer;
pub use integer::{
    Integer, deserialize_int, deserialize_int_optional, serialize_int, serialize_int_optional,
};

pub mod multi;
pub use multi::{deserialize_additional_data, deserialize_multi, serialize_multi};

mod mac_addr;
pub use mac_addr::MacAddr;

mod number;
pub use number::{
    Number, deserialize_number, deserialize_number_optional, serialize_number,
    serialize_number_optional,
};

mod vmid;
pub use vmid::VmId;
