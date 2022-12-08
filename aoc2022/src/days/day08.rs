use advent::Grid;
use hashbrown::HashSet;
use itertools::FoldWhile::{self, Continue, Done};
use itertools::Itertools;

fn is_visible(grid: &Grid<u32>, (x, y): (isize, isize)) -> bool {
	let tree = grid.get(x, y).unwrap();

	if (0..x).fold(true, |visible, i| {
		visible
			&& match grid.get(i, y) {
				Some(size) => tree > size,
				None => true,
			}
	}) {
		return true;
	}

	if (x + 1..grid.width.try_into().unwrap()).fold(true, |visible, i| {
		visible
			&& match grid.get(i, y) {
				Some(size) => tree > size,
				None => true,
			}
	}) {
		return true;
	}

	if (0..y).fold(true, |visible, j| {
		visible
			&& match grid.get(x, j) {
				Some(size) => tree > size,
				None => true,
			}
	}) {
		return true;
	}

	(y + 1..grid.height.try_into().unwrap()).fold(true, |visible, j| {
		visible
			&& match grid.get(x, j) {
				Some(size) => tree > size,
				None => true,
			}
	})
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

fn count_visible(grid: &Grid<u32>, (x, y): (isize, isize)) -> usize {
	let main_tree = *grid.get(x, y).unwrap();

	(0..x)
		.rev()
		.fold_while((0, 0), |(tree, count), i| {
			fold_direction(grid, main_tree, tree, count, i, y)
		})
		.into_inner()
		.1 * (x + 1..grid.width.try_into().unwrap())
		.fold_while((0, 0), |(tree, count), i| {
			fold_direction(grid, main_tree, tree, count, i, y)
		})
		.into_inner()
		.1 * (0..y)
		.rev()
		.fold_while((0, 0), |(tree, count), j| {
			fold_direction(grid, main_tree, tree, count, x, j)
		})
		.into_inner()
		.1 * (y + 1..grid.height.try_into().unwrap())
		.fold_while((0, 0), |(tree, count), j| {
			fold_direction(grid, main_tree, tree, count, x, j)
		})
		.into_inner()
		.1
}

pub fn pt1(input: &str) -> anyhow::Result<usize> {
	let grid = Grid::new(input);
	let mut visible: HashSet<(usize, usize)> = HashSet::new();

	for x in 0..grid.width {
		for y in 0..grid.height {
			if is_visible(&grid, (x.try_into()?, y.try_into()?)) {
				visible.insert((x, y));
			}
		}
	}

	Ok(visible.len())
}

pub fn pt2(input: &str) -> anyhow::Result<usize> {
	let grid = Grid::new(input);

	Ok(grid
		.iter()
		.map(|((x, y), _)| count_visible(&grid, (x, y)))
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
