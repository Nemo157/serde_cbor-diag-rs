#[macro_use]
mod utils;

macro_rules! integers {
    ($($ty:ident),*) => {
        serialize2diag! { $(
            $ty {
                zero(0 as $ty, "0")
                twenty_three(23 as $ty, "23")
                min($ty::min_value(), $ty::min_value().to_string())
                max($ty::max_value(), $ty::max_value().to_string())
            }
        )* }
    };
}

integers! {
    i8, i16, i32, i64, i128, u8, u16, u32, u64, u128
}
