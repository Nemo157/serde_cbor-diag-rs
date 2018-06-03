#[macro_use]
mod utils;

macro_rules! integers {
    ($($ty:ident),*) => {
        serialize2diag! { $(
            $ty {
                zero + zero_cbor(0 as $ty, "0")
                twenty_three + twenty_three_cbor(23 as $ty, "23")
                min + min_cbor($ty::min_value(), $ty::min_value().to_string())
                max + max_cbor($ty::max_value(), $ty::max_value().to_string())
            }
        )* }
    };
}

integers! {
    i8, i16, i32, i64, u8, u16, u32, u64
}

serialize2diag! {
    i128 {
        zero(0i128, "0")
        twenty_three(23i128, "23")
        min(i128::min_value(), "-170141183460469231731687303715884105728")
        max(i128::max_value(), "170141183460469231731687303715884105727")
    }

    u128 {
        zero(0u128, "0")
        twenty_three(23u128, "23")
        min(u128::min_value(), "0")
        max(u128::max_value(), "340282366920938463463374607431768211455")
    }
}
