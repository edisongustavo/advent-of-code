pub fn get_mut2<T>(v: &mut [T], i: usize, j: usize) -> (&mut T, &mut T) {
    if i == j {
        panic!("Can't return 2 mutable references to the same element");
    }
    let (start, end) = if i < j { (i, j) } else { (j, i) };

    let (first, second) = v.split_at_mut(start + 1);
    let left = &mut first[start];
    let right = &mut second[end - start - 1];

    if i < j {
        (left, right)
    } else {
        (right, left)
    }
}

pub fn split_into_slices<'a, T, const N: usize>(
    slice: &'a [T],
    indices: &[usize; N],
) -> Result<[&'a [T]; N], String> {
    let max_index = *indices.iter().max().ok_or("Empty slice".to_string())?;
    if max_index > slice.len() {
        return Err(format!(
            "Tried to split at index {max_index}, but slice has length {}",
            slice.len()
        ));
    }
    let ret = std::array::from_fn::<&[T], N, _>(|i| {
        let index = indices[i];
        let previous_index = if i > 0 { indices[i - 1] } else { 0 };
        &slice[previous_index..index]
    });
    Ok(ret)
}

#[cfg(test)]
mod tests {
    // use pretty_assertions::{assert_eq, assert_ne};
    use crate::containers::{get_mut2, split_into_slices};

    #[test]
    fn test_get_mut2() -> Result<(), String> {
        let mut source = vec![1, 2, 3, 4, 5];
        let (left, right) = get_mut2(source.as_mut_slice(), 0, 1);
        assert_eq!(*left, 1);
        assert_eq!(*right, 2);

        let (left, right) = get_mut2(source.as_mut_slice(), 1, 0);
        assert_eq!(*left, 2);
        assert_eq!(*right, 1);
        Ok(())
    }

    #[test]
    fn test_split_into_slices() -> Result<(), String> {
        let v = vec!["a", "b", "c", "d"];
        let ret = split_into_slices(&v, &[1, 2, 3])?;
        assert_eq!(ret, [
            &v[0..1],
            &v[1..2],
            &v[2..3],
        ]);
        Ok(())
    }

    #[test]
    fn test_split_into_slices_max_index() -> Result<(), String> {
        let v = vec!["a", "b", "c", "d"];
        let ret = split_into_slices(&v, &[1, 4])?;
        assert_eq!(ret, [
            &v[0..1],
            &v[1..4],
        ]);
        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_split_into_slices_error() {
        let v = vec!["a", "b", "c", "d"];
        split_into_slices(&v, &[20]).unwrap();
    }
}
