extern crate serde;
#[macro_use]
extern crate serde_derive;

#[macro_use]
mod utils;

#[derive(Serialize)]
struct UnitStruct;

#[derive(Serialize)]
struct NewtypeStruct<T>(T);

serialize2diag! {
    bool {
        true_(true, "true")
        false_(false, "false")
    }

    char {
        a('a', r#""a""#)
        space(' ', r#"" ""#)
        tab('\t', r#""\t""#)
        unicode_surrogate_pair('\u{10151}', r#""\u{10151}""#)
    }

    option {
        some(Some(5), "5")
        none({ let v: Option<i32> = None; v }, "null")
    }

    structs {
        unit(UnitStruct, "null")
        newtype(NewtypeStruct(5), "5")
    }
}
