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
    )* }
}
