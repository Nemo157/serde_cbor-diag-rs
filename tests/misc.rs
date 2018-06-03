extern crate serde;
#[macro_use]
extern crate serde_derive;

#[macro_use]
mod utils;

#[derive(Serialize)]
struct Unit;

#[derive(Serialize)]
struct Newtype<T>(T);

#[derive(Serialize)]
struct Tuple<T>(T, T);

#[derive(Serialize)]
struct Struct<T> {
    left: T,
    right: T,
}

#[derive(Serialize)]
enum Enum<T> {
    Unit,
    Newtype(T),
    Tuple(T, T),
    Struct { left: T, right: T },
}

serialize2diag! {
    bool {
        true_ + true_cbor(true, "true")
        false_ + false_cbor(false, "false")
    }

    char {
        a + a_cbor('a', r#""a""#)
        space + space_cbor(' ', r#"" ""#)
        tab + tab_cbor('\t', r#""\t""#)
        unicode + unicode_cbor('\u{10151}', r#""\u{10151}""#)
    }

    option {
        some + some_cbor(Some(5), "5")
        none + none_cbor({ let v: Option<i32> = None; v }, "null")
    }

    structs {
        unit + unit_cbor(Unit, "null")
        newtype + newtype_cbor(Newtype(5), "5")
        tuple + tuple_cbor(Tuple(5, 6), "[5, 6]")
        struct_ + struct_cbor(
            Struct { left: 5, right: 6 },
            r#"{ "left": 5, "right": 6 }"#)
    }

    enums {
        unit + unit_cbor({ let v: Enum<()> = Enum::Unit; v }, r#""Unit""#)
        newtype + newtype_cbor(Enum::Newtype(5), r#"{ "Newtype": 5 }"#)
        tuple + tuple_cbor(Enum::Tuple(5, 6), r#"{ "Tuple": [5, 6] }"#)
        struct_ + struct_cbor(
            Enum::Struct { left: 5, right: 6 },
            r#"{ "Struct": { "left": 5, "right": 6 } }"#)
    }
}
