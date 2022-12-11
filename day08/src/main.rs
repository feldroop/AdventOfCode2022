mod direction_view;
mod tree_height_history;

use std::{fs, iter};

use ndarray::{Array, Array2, ArrayView2, ArrayViewMut2};

use direction_view::DirectionViews;
use tree_height_history::TreeHeightHistory;

fn main() {
    let input = fs::read_to_string("input/day8.txt").unwrap();
    let lines: Vec<_> = input.lines().collect();

    let num_rows = lines.len();
    let num_columns = lines[0].len();
    let grid_shape = (num_rows, num_columns);

    let digit_iter = lines
        .into_iter()
        .flat_map(|line| line.chars())
        .filter_map(char_to_u8_digit);

    let tree_height_grid = Array::from_iter(digit_iter)
        .into_shape(grid_shape)
        .expect("Computed shape of grid should be valid");

    let is_visible_grid = Array2::from_elem(grid_shape, false);

    let tree_height_grid_views = DirectionViews::new(tree_height_grid);
    let mut is_visible_grid_views = DirectionViews::new(is_visible_grid);

    compute_from_all_directions(
        visibility,
        &tree_height_grid_views,
        &mut is_visible_grid_views,
    );

    let num_visible_trees = is_visible_grid_views
        .grid()
        .iter()
        .filter(|&&is_visible| is_visible)
        .count();

    println!("The number of visible trees is: {num_visible_trees}");

    let scenic_score_grid = Array2::from_elem(grid_shape, 1u32);
    let mut scenic_score_grid_views = DirectionViews::new(scenic_score_grid);

    compute_from_all_directions(
        scenic_score,
        &tree_height_grid_views,
        &mut scenic_score_grid_views,
    );

    let max_scenic_score = scenic_score_grid_views.grid().iter().max().unwrap();

    println!("The highest scenic score of any tree is {max_scenic_score}");
}

fn char_to_u8_digit(c: char) -> Option<u8> {
    c.to_digit(10).map(|digit| digit as u8)
}

fn compute_from_all_directions<F, T>(
    f: F,
    tree_height_grid: &DirectionViews<u8>,
    data_grid: &mut DirectionViews<T>,
) where
    F: Fn(ArrayView2<u8>, ArrayViewMut2<T>),
{
    f(
        tree_height_grid.left_to_right(),
        data_grid.left_to_right_mut(),
    );

    f(
        tree_height_grid.right_to_left(),
        data_grid.right_to_left_mut(),
    );

    f(
        tree_height_grid.top_to_bottom(),
        data_grid.top_to_bottom_mut(),
    );

    f(
        tree_height_grid.bottom_to_top(),
        data_grid.bottom_to_top_mut(),
    );
}

fn visibility(tree_height_grid: ArrayView2<u8>, mut is_visible_grid: ArrayViewMut2<bool>) {
    let rows = iter::zip(tree_height_grid.rows(), is_visible_grid.rows_mut());

    for (tree_height_row, mut is_visible_row) in rows {
        // outermost tree is always visible
        let mut max_seen_height = tree_height_row[0];
        is_visible_row[0] = true;

        let elements = iter::zip(tree_height_row, is_visible_row);

        for (&tree_height, is_visible) in elements {
            if tree_height > max_seen_height {
                *is_visible = true;
                max_seen_height = tree_height;
            }
        }
    }
}

fn scenic_score(tree_height_grid: ArrayView2<u8>, mut scenic_score_grid: ArrayViewMut2<u32>) {
    let rows = iter::zip(tree_height_grid.rows(), scenic_score_grid.rows_mut());

    for (tree_height_row, scenic_score_row) in rows {
        let mut tree_height_history = TreeHeightHistory::new();

        let elements = iter::zip(tree_height_row, scenic_score_row);

        for (&tree_height, scenic_score) in elements {
            *scenic_score *= tree_height_history.get_viewing_distance_and_update(tree_height);
        }
    }
}
