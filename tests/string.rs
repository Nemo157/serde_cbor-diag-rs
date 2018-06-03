#[macro_use]
mod utils;

serialize2diag! {
    empty + empty_cbor("", r#""""#)
    ietf + ietf_cbor("IETF", r#""IETF""#)
    quote_slash + quote_slash_cbor(r#""\"#, r#""\"\\""#)
    unicode_latin_supplement + unicode_latin_supplement_cbor(
        "\u{fc}",
        r#""\u{fc}""#)
    unicode_surrogate_pair + unicode_surrogate_pair_cbor(
        "\u{10151}",
        r#""\u{10151}""#)
}
