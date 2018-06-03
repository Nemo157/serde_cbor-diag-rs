#[macro_use]
mod utils;

serialize2diag! {
    empty("", r#""""#)
    ietf("IETF", r#""IETF""#)
    quote_slash(r#""\"#, r#""\"\\""#)
    unicode_latin_supplement("\u{fc}", r#""\u{fc}""#)
    unicode_surrogate_pair("\u{10151}", r#""\u{10151}""#)
}
