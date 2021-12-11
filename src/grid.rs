pub struct Grid<T> {
	width: usize,
	height: usize,
	cells: Vec<T>,
}

impl Grid<u32> {
	pub fn new(input: &str) -> Self {
		let height = input.lines().count();
		let mut width = None;

		let cells = input.lines().fold(Vec::new(), |mut vec, line| {
			if width.is_none() {
				width = Some(line.len());
			} else {
				assert!(width.unwrap() == line.len());
			}

			vec.extend(line.chars().map(|c| c.to_digit(10).unwrap()));

			vec
		});

		Grid {
			width: width.unwrap(),
			height,
			cells,
		}
	}
}

impl<T> Grid<T> {
	fn index(&self, x: isize, y: isize) -> Option<usize> {
		if x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize {
			return None;
		}

		let x = x as usize;
		let y = y as usize;

		let index = y * self.width + x;

		if index >= self.cells.len() {
			return None;
		}

		Some(index)
	}

	pub fn get(&self, x: isize, y: isize) -> Option<&T> {
		self.index(x, y).map(|index| &self.cells[index])
	}

	pub fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut T> {
		self.index(x, y).map(|index| &mut self.cells[index])
	}

	pub fn neighbors(&self, x: isize, y: isize) -> Neighbors<T> {
		Neighbors {
			grid: self,
			x,
			y,
			dx: -1,
			dy: -1,
		}
	}
}

impl<T1> Grid<T1> {
	pub fn map<F, T2>(self, f: F) -> Grid<T2>
	where
		F: Fn(T1) -> T2,
	{
		Grid {
			width: self.width,
			height: self.height,
			cells: self.cells.into_iter().map(f).collect(),
		}
	}
}

pub struct Neighbors<'a, T> {
	grid: &'a Grid<T>,
	x: isize,
	y: isize,
	dx: isize,
	dy: isize,
}

impl<'a, T> Iterator for Neighbors<'a, T> {
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		let mut next = None;

		while next.is_none() && self.dx < 2 && self.dy < 2 {
			next = self.grid.get(self.x + self.dx, self.y + self.dy);

			self.dx += 1;

			if self.dx > 1 {
				self.dx = -1;
				self.dy += 1;
			}

			if (self.dx, self.dy) == (0, 0) {
				self.dx = 1;
			}
		}

		next
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn simple_numeric_grids_can_be_created() {
		let grid = Grid::new("123\n456\n789\n");

		assert_eq!(grid.width, 3);
		assert_eq!(grid.height, 3);
		assert_eq!(grid.cells, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
	}

	#[test]
	fn simple_numeric_grids_must_be_rectangular() {
		let result = std::panic::catch_unwind(|| Grid::new("123\n456\n7890"));

		assert!(result.is_err());
	}

	#[test]
	fn a_grid_can_be_transformed() {
		let grid = Grid::new("12\n34\n");

		let grid = grid.map(|n| Some((n, n * n)));

		assert_eq!(
			grid.cells,
			vec![Some((1, 1)), Some((2, 4)), Some((3, 9)), Some((4, 16))],
		);
	}

	#[test]
	fn a_cell_can_be_retrieved() {
		let grid = Grid::new("12\n34\n");

		assert_eq!(grid.get(0, 0), Some(&1));
		assert_eq!(grid.get(1, 0), Some(&2));
		assert_eq!(grid.get(0, 1), Some(&3));
		assert_eq!(grid.get(1, 1), Some(&4));
		assert_eq!(grid.get(-1, 0), None);
		assert_eq!(grid.get(0, -1), None);
		assert_eq!(grid.get(2, 0), None);
		assert_eq!(grid.get(0, 2), None);
	}

	#[test]
	fn a_cell_can_be_mutated() {
		let mut grid = Grid::new("12\n34\n");

		grid.get_mut(0, 0).map(|cell| *cell = 5);

		assert_eq!(grid.cells, vec![5, 2, 3, 4]);
	}

	#[test]
	fn a_cells_neighbors_can_be_iterated_over() {
		let grid = Grid::new("123\n456\n789\n");

		let mut neighbors = grid.neighbors(1, 1);

		assert_eq!(neighbors.next(), Some(&1));
		assert_eq!(neighbors.next(), Some(&2));
		assert_eq!(neighbors.next(), Some(&3));
		assert_eq!(neighbors.next(), Some(&4));
		assert_eq!(neighbors.next(), Some(&6));
		assert_eq!(neighbors.next(), Some(&7));
		assert_eq!(neighbors.next(), Some(&8));
		assert_eq!(neighbors.next(), Some(&9));
		assert_eq!(neighbors.next(), None);
	}

	#[test]
	fn neightbors_outside_of_the_bounds_are_skipped() {
		let grid = Grid::new("12\n34\n");

		let mut neighbors = grid.neighbors(0, 0);

		assert_eq!(neighbors.next(), Some(&2));
		assert_eq!(neighbors.next(), Some(&3));
		assert_eq!(neighbors.next(), Some(&4));
		assert_eq!(neighbors.next(), None);
	}
}
