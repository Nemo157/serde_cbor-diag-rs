extern crate bytes;

use bytes::Bytes;

#[macro_use]
mod utils;

serialize2diag! {
    empty(Bytes::from_static(&[]), "h''")
    one_two_three_four(Bytes::from_static(&[1, 2, 3, 4]), "h'01020304'")
}
