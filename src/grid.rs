#[derive(Debug, Clone)]
pub struct Grid<T> {
	pub width: usize,
	pub height: usize,
	pub cells: Vec<T>,
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

impl<I, I2, T> From<I> for Grid<T>
where
	I: Iterator<Item = I2>,
	I2: Iterator<Item = T>,
{
	fn from(iter: I) -> Self {
		let mut height = 0;
		let cells = iter.fold(Vec::new(), |mut vec, line| {
			height += 1;
			vec.extend(line);

			vec
		});

		Grid {
			width: cells.len() / height,
			height,
			cells,
		}
	}
}

impl<T: Default + Clone> Grid<T> {
	pub fn new_with_size(width: usize, height: usize) -> Self {
		Grid {
			width,
			height,
			cells: vec![T::default(); width * height],
		}
	}

	pub fn add_empty_row(&mut self, row: usize) {
		match self.index(0, row as isize) {
			Some(index) => {
				self.cells
					.splice(index..index, vec![T::default(); self.width]);
			}
			None => {
				self.cells.extend(vec![T::default(); self.width]);
			}
		}

		self.height += 1;
	}

	pub fn add_empty_column(&mut self, column: usize) {
		for row in (0..self.height).rev() {
			let index = row * self.width + column;

			if index > self.cells.len() {
				self.cells.push(T::default());
			} else {
				self.cells.insert(index, T::default());
			}
		}

		self.width += 1;
	}

	pub fn add_row(&mut self, row: usize, value: T) {
		match self.index(0, row as isize) {
			Some(index) => {
				self.cells.splice(index..index, vec![value; self.width]);
			}
			None => {
				self.cells.extend(vec![value; self.width]);
			}
		}

		self.height += 1;
	}

	pub fn add_column(&mut self, column: usize, value: T) {
		for row in (0..self.height).rev() {
			let value = value.clone();
			let index = row * self.width + column;

			if index > self.cells.len() {
				self.cells.push(value);
			} else {
				self.cells.insert(index, value);
			}
		}

		self.width += 1;
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

	pub fn insert(&mut self, x: isize, y: isize, v: T) {
		let i = self.index(x, y).unwrap();
		self.cells[i] = v;
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

	pub fn iter(&self) -> Iter<T> {
		Iter {
			cells: self.cells.iter(),
			x: 0,
			y: 0,
			width: self.width,
		}
	}

	pub fn iter_mut(&mut self) -> IterMut<T> {
		IterMut {
			cells: self.cells.iter_mut(),
			x: 0,
			y: 0,
			width: self.width,
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

pub struct Iter<'a, T> {
	cells: std::slice::Iter<'a, T>,
	x: usize,
	y: usize,
	width: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
	type Item = ((isize, isize), &'a T);

	fn next(&mut self) -> Option<Self::Item> {
		let x = self.x.try_into().unwrap();
		let y = self.y.try_into().unwrap();

		self.x += 1;

		if self.x >= self.width {
			self.x = 0;
			self.y += 1;
		}

		self.cells.next().map(|cell| ((x, y), cell))
	}
}

pub struct IterMut<'a, T> {
	cells: std::slice::IterMut<'a, T>,
	x: usize,
	y: usize,
	width: usize,
}

impl<'a, T> Iterator for IterMut<'a, T> {
	type Item = ((isize, isize), &'a mut T);

	fn next(&mut self) -> Option<Self::Item> {
		let x = self.x.try_into().unwrap();
		let y = self.y.try_into().unwrap();

		self.x += 1;

		if self.x >= self.width {
			self.x = 0;
			self.y += 1;
		}

		self.cells.next().map(|cell| ((x, y), cell))
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

	#[test]
	fn a_grid_can_be_iterated() {
		let grid = Grid::new("12\n34\n");

		let mut iterator = grid.iter();

		assert_eq!(iterator.next(), Some(((0, 0), &1)));
		assert_eq!(iterator.next(), Some(((1, 0), &2)));
		assert_eq!(iterator.next(), Some(((0, 1), &3)));
		assert_eq!(iterator.next(), Some(((1, 1), &4)));
		assert_eq!(iterator.next(), None);
	}

	#[test]
	fn a_grid_can_be_iterated_mutably() {
		let mut grid = Grid::new("12\n34\n");

		grid.iter_mut().for_each(|(_, cell)| *cell += 1);

		assert_eq!(grid.cells, vec![2, 3, 4, 5]);
	}

	#[test]
	fn a_grid_can_be_created_from_iterators() {
		let grid: Grid<_> = "123\n456\n789\n"
			.lines()
			.map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
			.into();

		assert_eq!(grid.width, 3);
		assert_eq!(grid.height, 3);
		assert_eq!(grid.cells, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
	}
}
