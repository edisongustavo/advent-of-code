use std::fmt::Formatter;
use ndarray::Array2;
use itertools::{enumerate, Itertools};

pub fn fmt_array2d(f: &mut Formatter, elems: &Array2<u8>) -> std::fmt::Result {
    for (i, row) in enumerate(elems.rows()) {
        let row_string = row.to_vec().iter().map(|num| num.to_string()).join("");
        f.write_str(row_string.as_str())?;
        if i < elems.nrows() - 1 {
            f.write_str("\n")?;
        }
    }
    Ok(())
}
