#[macro_use]
mod utils;

serialize2diag! {
    f32 {
        zero(0.0f32, "0.0")
        neg_zero(-0.0f32, "-0.0")
        twenty_three(23.0f32, "23.0")
        nan(::std::f32::NAN, "NaN")
        infinity(::std::f32::INFINITY, "Infinity")
        neg_infinity(::std::f32::NEG_INFINITY, "-Infinity")
        big(3.4028235e38f32, "3.4028235e38")
        tiny(5.9604645e-8f32, "5.9604645e-8")
    }

    f64 {
        zero(0.0f64, "0.0")
        neg_zero(-0.0f64, "-0.0")
        twenty_three(23.0f64, "23.0")
        nan(::std::f64::NAN, "NaN")
        infinity(::std::f64::INFINITY, "Infinity")
        neg_infinity(::std::f64::NEG_INFINITY, "-Infinity")
        big(3.4028234663852887e38f64, "3.4028234663852887e38")
        tiny(5.960464477539063e-8f64, "5.960464477539063e-8")
    }
}
