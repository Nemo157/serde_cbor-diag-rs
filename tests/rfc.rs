/// Test cases from RFC 7049 Appendix A

#[macro_use]
mod utils;

cbor2diag! {
    integer {
        zero(b"00", "0")
        one(b"01", "1")
        ten(b"0a", "10")
        twenty_three(b"17", "23")
        twenty_four(b"1818", "24")
        twenty_five(b"1819", "25")
        one_hundred(b"1864", "100")
        one_thousand(b"1903e8", "1000")
        one_million(b"1a000f4240", "1000000")
        one_long_billion(b"1b000000e8d4a51000", "1000000000000")
        u64_max(b"1bffffffffffffffff", "18446744073709551615")

        // u128 is not supported by serde_cbor::Value
        //
        // u64_max_plus_one(b"c249010000000000000000", "18446744073709551616")

        // TODO: Broken, don't know where exactly
        //
        // neg_u64_max(b"3bffffffffffffffff", "-18446744073709551616")

        neg_one(b"20", "-1")
        neg_ten(b"29", "-10")
        neg_one_hundred(b"3863", "-100")
        neg_one_thousand(b"3903e7", "-1000")
    }

    float {
        float_zero(b"f90000", "0.0")
        float_neg_zero(b"f98000", "-0.0")
        float_one(b"f93c00", "1.0")
        float_one_point_one(b"fb3ff199999999999a", "1.1")
        float_one_point_five(b"f93e00", "1.5")
        float_sixty_five_thousand_five_hundred_and_four(b"f97bff", "65504.0")
        float_one_hundred_thousand(b"fa47c35000", "100000.0")

        // `dtoa`'s exponential notation is slightly different to what is shown in
        // the RFC, since this is not a real serialization format that's fine.
        float_big(b"fa7f7fffff", "3.4028234663852887e38")
        float_one_e_300(b"fb7e37e43c8800759c", "1e300")
        float_tiny(b"f90001", "5.960464477539063e-8")
        float_small(b"f90400", "0.00006103515625")

        float_neg_four(b"f9c400", "-4.0")
        float_neg_four_point_one(b"fbc010666666666666", "-4.1")

        float_infinity_half(b"f97c00", "Infinity")
        float_infinity_single(b"fa7f800000", "Infinity")
        float_infinity_double(b"fb7ff0000000000000", "Infinity")

        float_nan_half(b"f97e00", "NaN")
        float_nan_single(b"fa7fc00000", "NaN")
        float_nan_double(b"fb7ff8000000000000", "NaN")

        float_neg_infinity_half(b"f9fc00", "-Infinity")
        float_neg_infinity_single(b"faff800000", "-Infinity")
        float_neg_infinity_double(b"fbfff0000000000000", "-Infinity")
    }

    simple {
        bool_false(b"f4", "false")
        bool_true(b"f5", "true")

        // TODO serde_cbor::Value's handling of bare null and undefined seems off
        //
        // null(b"f6", "null")
        // undefined(b"f7", "undefined")

        // serde_cbor::Value doesn't support unassigned simple values
        //
        // simple_sixteen(b"f0", "simple(16)")
        // simple_twenty_fourc(b"f818", "simple(24)")
        // simple_two_hundred_fifty_five(b"f8ff", "simple(255)")
    }

    tagged {
        // Note: serde doesn't have any way to support tagging of data, the tags
        // that the RFC included on the following diagnostic notations have all been
        // dropped.
        date(
            b"c074323031332d30332d32315432303a30343a30305a",
            r#""2013-03-21T20:04:00Z""#)
        integer(b"c11a514b67b0", "1363896240")
        float(b"c1fb41d452d9ec200000", "1363896240.5")
        base16_bytes(b"d74401020304", "h'01020304'")
        encoded_cbor_item(b"d818456449455446", "h'6449455446'")
        url(
            b"d82076687474703a2f2f7777772e6578616d706c652e636f6d",
            r#""http://www.example.com""#)
    }

    byte_string {
        empty(b"40", "h''")
        one_two_three_four(b"4401020304", "h'01020304'")

        // serde doesn't support streaming byte strings
        //
        // indefinite(b"5f42010243030405ff", "(_ h'0102', h'030405')")

    }

    string {
        empty(b"60", r#""""#)
        a(b"6161", r#""a""#)
        ietf(b"6449455446", r#""IETF""#)
        quote_slash(b"62225c", r#""\"\\""#)

        // Note: This is using Rusts unicode escaping, this is quite different
        // to the Javascript based escaping shown in the RFC.
        unicode_latin_supplement(b"62c3bc", r#""\u{fc}""#)
        unicode_surrogate_pair(b"64f0908591", r#""\u{10151}""#)

        // serde doesn't support streaming strings
        //
        // indefinite(b"7f657374726561646d696e67ff", r#"(_ "strea", "ming")"#)

    }

    array {
        empty(b"80", "[]")
        integers(b"83010203", "[1, 2, 3]")
        heterogenous(b"8301820203820405", "[1, [2, 3], [4, 5]]")
        large(
            b"98190102030405060708090a0b0c0d0e0f101112131415161718181819",
            "[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, \
              19, 20, 21, 22, 23, 24, 25]")

        // serde_cbor::Value doesn't support indefinite arrays
        //
        // indefinite_empty(b"9fff", "[_ ]")
        // indefinite_with_indefinite_suffix(
        //     b"9f018202039f0405ffff",
        //     "[_ 1, [2, 3], [_ 4, 5]]")
        // indefinite_with_definite_suffix(
        //     b"9f01820203820405ff",
        //     "[_ 1, [2, 3], [4, 5]]")
        // definite_with_indefinite_suffix(
        //     b"83018202039f0405ff",
        //     "[1, [2, 3], [_ 4, 5]]")
        // definite_with_indefinite_middle(
        //     b"83019f0203ff820405",
        //     "[1, [_ 2, 3], [4, 5]]")
        // indefinite_large(
        //     b"9f0102030405060708090a0b0c0d0e0f101112131415161718181819ff",
        //     "[_ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, \
        //       18, 19, 20, 21, 22, 23, 24, 25]")
    }

    map {
        empty(b"a0", "{ }")
        integers(b"a201020304", "{ 1: 2, 3: 4 }")
        strings(
            b"a56161614161626142616361436164614461656145",
            r#"{ "a": "A", "b": "B", "c": "C", "d": "D", "e": "E" }"#)
    }

    compound {
        map_including_array(
            b"a26161016162820203",
            r#"{ "a": 1, "b": [2, 3] }"#)
        array_including_map(
            b"826161a161626163",
            r#"["a", { "b": "c" }]"#)

        // serde_cbor::Value doesn't support indefinite maps
        //
        // indefinite_map_with_indefinite_array_suffix(
        //     b"bf61610161629f0203ffff",
        //     r#"{_ "a": 1, "b": [_ 2, 3] }"#)
        // definite_array_with_indefinite_map_suffix(
        //     b"826161bf61626163ff",
        //     r#"["a", {_ "b": "c"}]"#)
        // indefinite_map_with_heterogenous_values(
        //     b"bf6346756ef563416d7421ff",
        //     r#"{_ "Fun": true, "Amt": -2 }"#)
    }
}
