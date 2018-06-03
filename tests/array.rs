extern crate serde;

use serde::ser::{self, SerializeSeq};

#[macro_use]
mod utils;

struct Indefinite<T>(Vec<T>);

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

    indefinite {
        empty(
            { let v: Indefinite<i32> = Indefinite(vec![]); v },
            "[_ ]")

        integers(
            Indefinite(vec![1, 2, 3]),
            "[_ 1, 2, 3]")

        arrays(
            Indefinite(vec![Indefinite(vec![1, 2]), Indefinite(vec![3, 4])]),
            "[_ [_ 1, 2], [_ 3, 4]]")
    }
}

impl<T> ser::Serialize for Indefinite<T> where T: ser::Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        let mut seq = serializer.serialize_seq(None)?;
        for value in &self.0 {
            seq.serialize_element(value)?;
        }
        seq.end()
    }
}
