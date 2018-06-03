#[macro_use]
extern crate maplit;
extern crate serde;

use std::collections::BTreeMap;

use serde::ser::{self, SerializeMap};

#[macro_use]
mod utils;

struct Indefinite<K, V>(BTreeMap<K, V>);

serialize2diag! {
    map {
        empty({ let v: BTreeMap<(), ()> = btreemap!{ }; v }, "{ }")
        integers(btreemap!{ 1 => 2, 3 => 4 }, "{ 1: 2, 3: 4 }")
        strings(
            btreemap!{ "a" => "A", "b" => "B" },
            r#"{ "a": "A", "b": "B" }"#)
    }

    indefinite {
        empty(
            { let v: Indefinite<(), ()> = Indefinite(btreemap!{ }); v },
            "{_ }")
        integers(
            Indefinite(btreemap!{ 1 => 2, 3 => 4 }),
            "{_ 1: 2, 3: 4 }")
        strings(
            Indefinite(btreemap!{ "a" => "A", "b" => "B" }),
            r#"{_ "a": "A", "b": "B" }"#)
    }
}

impl<K, V> ser::Serialize for Indefinite<K, V>
where
    K: ser::Serialize,
    V: ser::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut seq = serializer.serialize_map(None)?;
        for (key, value) in &self.0 {
            seq.serialize_key(key)?;
            seq.serialize_value(value)?;
        }
        seq.end()
    }
}
