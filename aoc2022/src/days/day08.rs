use advent::Grid;
use itertools::FoldWhile::{self, Continue, Done};
use itertools::Itertools;

fn fold_visible_count(
	grid: &Grid<u32>,
	main_tree: u32,
	visible: bool,
	x: isize,
	y: isize,
) -> FoldWhile<bool> {
	if visible
		&& match grid.get(x, y) {
			Some(&size) => main_tree > size,
			None => true,
		} {
		Continue(true)
	} else {
		Done(false)
	}
}

fn fold_direction(
	grid: &Grid<u32>,
	main_tree: u32,
	tree: u32,
	count: usize,
	x: isize,
	y: isize,
) -> FoldWhile<(u32, usize)> {
	let size = *grid.get(x, y).unwrap();
	if main_tree <= size {
		Done((size, count + 1))
	} else {
		Continue((size.max(tree), count + 1))
	}
}

pub fn pt1(input: &str) -> anyhow::Result<usize> {
	let grid = Grid::new(input);

	Ok(grid
		.iter()
		.map(|((x, y), &main_tree)| {
			(0..x)
				.fold_while(true, |visible, i| {
					fold_visible_count(&grid, main_tree, visible, i, y)
				})
				.into_inner() || (x + 1..grid.width.try_into().unwrap())
				.fold_while(true, |visible, i| {
					fold_visible_count(&grid, main_tree, visible, i, y)
				})
				.into_inner() || (0..y)
				.fold_while(true, |visible, j| {
					fold_visible_count(&grid, main_tree, visible, x, j)
				})
				.into_inner() || (y + 1..grid.height.try_into().unwrap())
				.fold_while(true, |visible, j| {
					fold_visible_count(&grid, main_tree, visible, x, j)
				})
				.into_inner()
		})
		.filter(|&visible| visible)
		.count())
}

pub fn pt2(input: &str) -> anyhow::Result<usize> {
	let grid = Grid::new(input);

	Ok(grid
		.iter()
		.map(|((x, y), &main_tree)| {
			(0..x)
				.rev()
				.fold_while((0, 0), |(tree, count), i| {
					fold_direction(&grid, main_tree, tree, count, i, y)
				})
				.into_inner()
				.1 * (x + 1..grid.width.try_into().unwrap())
				.fold_while((0, 0), |(tree, count), i| {
					fold_direction(&grid, main_tree, tree, count, i, y)
				})
				.into_inner()
				.1 * (0..y)
				.rev()
				.fold_while((0, 0), |(tree, count), j| {
					fold_direction(&grid, main_tree, tree, count, x, j)
				})
				.into_inner()
				.1 * (y + 1..grid.height.try_into().unwrap())
				.fold_while((0, 0), |(tree, count), j| {
					fold_direction(&grid, main_tree, tree, count, x, j)
				})
				.into_inner()
				.1
		})
		.max()
		.unwrap())
}

advent::problem!(
	r#"
		30373
		25512
		65332
		33549
		35390
    "#,
	21,
	8,
);
