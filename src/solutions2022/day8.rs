use crate::files::lines;
use itertools::Itertools;
use ndarray::prelude::*;
use std::cmp::max;
use std::fmt::{Debug, Formatter};
use std::iter::zip;
use crate::array;

type PuzzleResult = usize;

pub fn day8() -> (PuzzleResult, PuzzleResult) {
    let lines_of_file = lines("inputs/2022/day8.txt");
    inner(lines_of_file)
}

struct Grid {
    elems: Array2<u8>,
}

impl Grid {
    fn count_inner_visible_trees(&self) -> usize {
        let visible_trees = self.compute_visible_trees_mask();
        let shape = self.elems.shape();
        let inner = visible_trees.slice(s![1..shape[0] - 1, 1..shape[1] - 1]);
        inner.iter().filter(|elem| **elem).count()
    }

    fn find_max_scenic_score(&self) -> usize {
        *self.compute_scenic_scores().iter().max().unwrap()
    }

    fn compute_visible_trees(&self) -> Array3<usize> {
        let nrows = self.elems.nrows();
        let ncols = self.elems.ncols();
        let mut visible_trees = Array3::zeros((nrows, ncols, 4));
        let row_slices = (0..(nrows)).map(|row| (s![row, ..], s![row, .., ..2]));
        let col_slices = (0..(ncols)).map(|col| (s![.., col], s![.., col, 2..]));
        let slices = row_slices.chain(col_slices);
        for (arr_slice, visible_trees_slice) in slices {
            let trees_array = self.elems.slice(arr_slice);
            let mut visible_trees_in_arr = visible_trees.slice_mut(visible_trees_slice);
            Self::fill_visible_trees(trees_array, &mut visible_trees_in_arr);
        }
        visible_trees
    }
    fn compute_scenic_scores(&self) -> Array2<usize> {
        let visible_trees = self.compute_visible_trees();
        let ret = visible_trees.map_axis(Axis(2), |f| f.product());
        ret
    }

    fn fill_visible_trees(
        arr: ArrayView<u8, Ix1>,
        visible_trees: &mut ArrayViewMut<usize, Ix2>,
    ) {
        // dbg!(arr, &visible_trees);
        for (i, tree) in arr.iter().enumerate() {
            let left = arr.slice(s![..i]);
            let right = arr.slice(s![i + 1..]);
            let left_count = left
                .iter()
                .rev()
                .take_while_inclusive(|i_tree| tree > i_tree)
                .count();
            let right_count = right
                .iter()
                .take_while_inclusive(|i_tree| tree > i_tree)
                .count();
            // dbg!(i, tree);
            // dbg!(left);
            // dbg!(right);
            // dbg!(left_count);
            // dbg!(right_count);
            let val = array![left_count, right_count];
            let mut foo = visible_trees.index_axis_mut(Axis(0), i);
            foo.assign(&val);
            // dbg!(&visible_trees);
        }
        // dbg!(&visible_trees);
    }

    fn compute_visible_trees_mask(&self) -> Array2<bool> {
        let nrows = self.elems.nrows();
        let ncols = self.elems.ncols();
        let mut visible_trees: Array2<bool> = Array2::default((nrows, ncols));
        visible_trees.fill(false);
        visible_trees.slice_mut(s![0, ..]).fill(true);
        visible_trees.slice_mut(s![nrows - 1, ..]).fill(true);
        visible_trees.slice_mut(s![.., 0]).fill(true);
        visible_trees.slice_mut(s![.., ncols - 1]).fill(true);

        let row_slices = (1..(nrows - 1)).map(|row| s![row, ..]);
        let col_slices = (1..(ncols - 1)).map(|col| s![.., col]);
        let slices = row_slices.chain(col_slices);
        for slice in slices {
            let arr = self.elems.slice(slice);
            let mut visible = visible_trees.slice_mut(slice);
            Self::mark_visible_trees_in_both_directions(arr, &mut visible);
        }
        visible_trees
    }

    fn mark_visible_trees_in_both_directions(
        s: ArrayView<u8, Ix1>,
        visible: &mut ArrayViewMut<bool, Ix1>,
    ) {
        Self::mark_visible_trees_from_left(s.iter(), visible.iter_mut());
        Self::mark_visible_trees_from_left(s.iter().rev(), visible.iter_mut().rev());
    }

    fn mark_visible_trees_from_left<'a>(
        mut iter: impl Iterator<Item = &'a u8>,
        field: impl Iterator<Item = &'a mut bool>,
    ) {
        let mut max_value = *iter.next().unwrap();
        for (val, is_visible) in zip(iter, field.skip(1)) {
            if *val > max_value {
                *is_visible = true;
            }
            max_value = max(max_value, *val);
        }
    }

    fn parse_lines(lines: Vec<String>) -> Result<Grid, String> {
        if lines.is_empty() || lines[0].is_empty() {
            return Err("Invalid lines!".to_string());
        }
        let mut arr = Array2::zeros((lines.len(), lines[0].len()));
        for (i, line) in lines.iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                let val = c.to_string().parse::<u8>().map_err(|err| err.to_string())?;
                arr[[i, j]] = val;
            }
        }
        let grid = Grid { elems: arr };
        Ok(grid)
    }
}

impl Grid {
    fn number_of_edge_trees(&self) -> usize {
        return (self.elems.nrows() + self.elems.ncols()) * 2 - 4;
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let elems = &self.elems;
        array::fmt_array2d(f, elems)?;
        Ok(())
    }
}

fn inner(lines: Vec<String>) -> (PuzzleResult, PuzzleResult) {
    let grid = Grid::parse_lines(lines).unwrap();
    let part1 = grid.number_of_edge_trees() + grid.count_inner_visible_trees();
    let part2 = grid.find_max_scenic_score();
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::strings::SkipEmptyLines;
    use itertools::Itertools;
    use ndarray::array;
    use pretty_assertions::assert_eq;
    use textwrap::dedent;

    fn to_vec(f: Vec<&str>) -> Vec<String> {
        f.into_iter().map(|s| String::from(s)).collect_vec()
    }

    #[test]
    fn test_part1_inner() -> Result<(), String> {
        let grid_string = dedent(
            "
            30373
            25512
            65332
            33549
            35390",
        )
        .skip_empty_start_lines();
        let actual = inner(to_vec(grid_string.lines().collect_vec())).0;
        assert_eq!(actual, 21);

        Ok(())
    }

    #[test]
    fn test_part2_inner() -> Result<(), String> {
        let grid_string = dedent(
            "
            30373
            25512
            65332
            33549
            35390",
        )
        .skip_empty_start_lines();
        let actual = inner(to_vec(grid_string.lines().collect_vec())).1;
        assert_eq!(actual, 8);

        Ok(())
    }

    fn grid() -> Grid {
        Grid {
            elems: array![
                [3, 0, 3, 7, 3],
                [2, 5, 5, 1, 2],
                [6, 5, 3, 3, 2],
                [3, 3, 5, 4, 9],
                [3, 5, 3, 9, 0],
            ],
        }
    }

    #[test]
    fn test_parse_grid() -> Result<(), String> {
        let grid_string = dedent(
            "
            30373
            25512
            65332
            33549
            35390",
        )
        .skip_empty_start_lines();
        let lines = to_vec(grid_string.lines().collect_vec());
        let grid = Grid::parse_lines(lines)?;
        assert_eq!(
            grid.elems,
            array![
                [3, 0, 3, 7, 3],
                [2, 5, 5, 1, 2],
                [6, 5, 3, 3, 2],
                [3, 3, 5, 4, 9],
                [3, 5, 3, 9, 0],
            ]
        );
        let actual = format!("{grid:?}");
        assert_eq!(actual, grid_string);
        Ok(())
    }

    #[test]
    fn test_find_inner_visible_trees() -> Result<(), String> {
        let grid = grid();
        assert_eq!(grid.number_of_edge_trees(), 16);
        assert_eq!(grid.count_inner_visible_trees(), 5);
        Ok(())
    }

    // #[test]
    // fn test_foo() -> Result<(), String> {
    //     let grid_string = dedent(
    //         "
    //         124532536362633
    //         314423354222564
    //         435436254635333
    //         154126533266547
    //         335533235452645
    //         241244443464555",
    //     )
    //     .skip_empty_start_lines();
    //     let lines = to_vec(grid_string.lines().collect_vec());
    //     let grid = Grid::parse_lines(lines)?;
    //
    //     let mask_bool = grid.compute_visible_trees_mask();
    //     let mask = Array::from_iter(
    //         mask_bool
    //             .clone()
    //             .iter_mut()
    //             .map(|val| if *val { 1u8 } else { 0u8 }),
    //     );
    //     let shape = mask_bool.shape();
    //     let mask: Array2<u8> = mask
    //         .into_shape((shape[0], shape[1]))
    //         .map_err(|err| err.to_string())?
    //         .into_dimensionality::<Ix2>()
    //         .map_err(|err| err.to_string())?;
    //     let grid_mask = Grid { elems: mask };
    //     println!("{grid_mask:?}");
    //     // .reshape();
    //     // Array2::from
    //     // for val in mask.iter_mut() {
    //     //
    //     // }
    //     // dbg!();
    //     assert_eq!(grid.count_inner_visible_trees(), 5);
    //     Ok(())
    // }

    #[test]
    fn test_compute_scenic_scores() -> Result<(), String> {
        let grid = grid();
        let scenic_scores = grid.compute_scenic_scores();
        assert_eq!(
            scenic_scores,
            array![
                [4, 3, 4, 8, 4],
                [3, 4, 6, 4, 4],
                [8, 7, 4, 5, 3],
                [3, 4, 7, 6, 8],
                [2, 5, 3, 8, 2],
            ]
        );
        Ok(())
    }

    #[test]
    fn test_compute_visible_trees() -> Result<(), String> {
        let grid = grid();
        let visible_trees = grid.compute_visible_trees();
        assert_eq!(
            visible_trees,
            array![
                [[0, 2, 0, 2], /* 3 */ [1, 1, 0, 1], /* 0 */ [2, 1, 0, 1], /* 3 */ [3, 1, 0, 4], /* 7 */ [1, 0, 0, 3] /* 3 */ ],
                [[0, 1, 1, 1], /* 2 */ [1, 1, 1, 1], /* 5 */ [1, 2, 1, 2], /* 5 */ [1, 1, 1, 1], /* 1 */ [2, 0, 1, 1] /* 2 */ ],
                [[0, 4, 2, 2], /* 6 */ [1, 3, 1, 2], /* 5 */ [1, 1, 1, 1], /* 3 */ [1, 1, 2, 1], /* 3 */ [1, 0, 1, 1] /* 2 */ ],
                [[0, 1, 1, 1], /* 3 */ [1, 1, 1, 1], /* 3 */ [2, 2, 2, 1], /* 5 */ [1, 1, 3, 1], /* 4 */ [4, 0, 3, 1] /* 9 */ ],
                [[0, 1, 1, 0], /* 3 */ [1, 2, 2, 0], /* 5 */ [1, 1, 1, 0], /* 3 */ [3, 1, 4, 0], /* 9 */ [1, 0, 1, 0] /* 0 */ ],
            ]
        );
        Ok(())
    }
}
