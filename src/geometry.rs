pub fn contains(left: (i32, i32), right: (i32, i32)) -> bool {
    let (x0, y0) = left;
    let (x1, y1) = right;
    x0 <= x1 && y0 >= y1
}

pub fn overlaps(left: (i32, i32), right: (i32, i32)) -> bool {
    let (a0, a1) = left;
    let (b0, b1) = right;
    a1 >= b0 && a0 <= b1
}
