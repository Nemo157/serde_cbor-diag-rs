#[macro_use]
mod utils;

serialize2diag! {
    f32 {
        zero + zero_cbor(0.0f32, "0.0")
        neg_zero + neg_zero_cbor(-0.0f32, "-0.0")
        twenty_three + twenty_three_cbor(23.0f32, "23.0")
        nan + nan_cbor(::std::f32::NAN, "NaN")
        infinity + infinity_cbor(::std::f32::INFINITY, "Infinity")
        neg_infinity + neg_infinity_cbor(::std::f32::NEG_INFINITY, "-Infinity")
        big(3.4028235e38f32, "3.4028235e38")
        tiny(5.9604645e-8f32, "5.9604645e-8")
    }

    f64 {
        zero + zero_cbor(0.0f64, "0.0")
        neg_zero + neg_zero_cbor(-0.0f64, "-0.0")
        twenty_three + twenty_three_cbor(23.0f64, "23.0")
        nan + nan_cbor(::std::f64::NAN, "NaN")
        infinity + infinity_cbor(::std::f64::INFINITY, "Infinity")
        neg_infinity + neg_infinity_cbor(::std::f64::NEG_INFINITY, "-Infinity")
        big + big_cbor(3.4028234663852887e38f64, "3.4028234663852887e38")
        tiny + tiny_cbor(5.960464477539063e-8f64, "5.960464477539063e-8")
    }
}
