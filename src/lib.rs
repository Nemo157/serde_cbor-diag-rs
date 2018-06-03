#[macro_use]
extern crate serde;
extern crate serde_cbor;

pub mod error;
pub mod ser;
pub use serde_cbor::value;

#[doc(inline)]
pub use self::error::{Error, Result};
#[doc(inline)]
pub use self::ser::{
    to_string_pretty, to_vec_pretty, to_writer_pretty, Serializer,
};
