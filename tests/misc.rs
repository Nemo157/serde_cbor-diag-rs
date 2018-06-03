#[macro_use]
mod utils;

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
}
