pub extern crate hex;
pub extern crate serde_cbor;
pub extern crate serde_cbor_diag;

#[macro_export]
macro_rules! cbor2diag {
    ($($name:ident($cbor:expr, $diag:expr))*) => { $(
        #[test]
        fn $name() {
            use $crate::utils::hex::decode;
            use $crate::utils::serde_cbor::{from_slice, Value};
            use $crate::utils::serde_cbor_diag::to_string_pretty;
            let bytes = decode(&$cbor[..]).unwrap();
            let value: Value = from_slice(&bytes).unwrap();
            let diag = to_string_pretty(&value).unwrap();
            assert_eq!(diag, $diag);
        }
    )* };

    ($($name:ident { $($inner:tt)* })*) => { $(
        mod $name {
            cbor2diag! {
                $($inner)*
            }
        }
    )* };
}

#[macro_export]
macro_rules! serialize2diag {
    ($($name:ident($value:expr, $diag:expr))*) => { $(
        #[test]
        fn $name() {
            use $crate::utils::serde_cbor_diag::to_string_pretty;
            let diag = to_string_pretty(&$value).unwrap();
            assert_eq!(diag, $diag);
        }
    )* };

    ($($name:ident { $($inner:tt)* })*) => { $(
        mod $name {
            #[allow(unused_imports)]
            use super::*;
            serialize2diag! {
                $($inner)*
            }
        }
    )* };
}
