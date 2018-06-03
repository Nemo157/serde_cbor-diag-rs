#[macro_use]
mod utils;

serialize2diag! {
    tuple {
        // empty tuple == null
        integers((1, 2, 3), "[1, 2, 3]")
        arrays(((1, 2), (3, 4)), "[[1, 2], [3, 4]]")
    }

    array {
        empty({ let v: [i32; 0] = []; v }, "[]")
        integers([1, 2, 3], "[1, 2, 3]")
        arrays([[1, 2], [3, 4]], "[[1, 2], [3, 4]]")
    }

    slice {
        empty({ let v: &[i32] = &[][..]; v }, "[]")
        integers(&[1, 2, 3][..], "[1, 2, 3]")
        arrays(&[&[1, 2][..], &[3, 4, 5][..]][..], "[[1, 2], [3, 4, 5]]")
    }

    vec {
        empty({ let v: Vec<i32> = vec![]; v }, "[]")
        integers(vec![1, 2, 3], "[1, 2, 3]")
        arrays(vec![vec![1, 2], vec![3, 4]], "[[1, 2], [3, 4]]")
    }
}
