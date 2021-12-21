use std::cmp::Reverse;

use hashbrown::HashSet;

pub fn pt1(input: &str) -> anyhow::Result<u32> {
	let grid = advent::Grid::new(input);

	let mut priority_queue = std::collections::BinaryHeap::new();
	let mut visited = HashSet::new();

	priority_queue.push((Reverse(0), (0, 0)));

	while let Some((Reverse(cost), (x, y))) = priority_queue.pop() {
		if !visited.insert((x, y)) {
			continue;
		}

		for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
			let (nx, ny) = (x + dx, y + dy);

			if let Some(tile_cost) = grid.get(nx, ny) {
				if nx == (grid.width - 1).try_into().unwrap()
					&& ny == (grid.height - 1).try_into().unwrap()
				{
					return Ok(cost + tile_cost);
				}

				priority_queue.push((Reverse(cost + tile_cost), (nx, ny)));
			}
		}
	}

	Err(anyhow::anyhow!("No path found"))
}

pub fn pt2(input: &str) -> anyhow::Result<u32> {
	let grid: advent::Grid<u32> = input
		.lines()
		.map(|line| line.chars().map(|tile| tile.to_digit(10).unwrap()))
		.into();

	let mut large_grid = advent::Grid::new_with_size(grid.width * 5, grid.height * 5);


	(0..(5 * grid.width)).for_each(|x| {
		(0..(5 * grid.height)).for_each(|y| {
			let cost = grid
				.get((x % grid.width) as isize, (y % grid.height) as isize)
				.unwrap() + (x / grid.width) as u32
				+ (y / grid.height) as u32;

			large_grid.insert(x as isize, y as isize, (cost - 1) % 9 + 1);
		})
	});


	let grid = large_grid;

	let mut priority_queue = std::collections::BinaryHeap::new();
	let mut visited = HashSet::new();

	priority_queue.push((Reverse(0), 0, 0));
	let end = ((grid.width - 1) as isize, (grid.height - 1) as isize);

	while let Some((Reverse(cost), x, y)) = priority_queue.pop() {
		if !visited.insert((x, y)) {
			continue;
		}

		for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
			let (x, y) = (x + dx, y + dy);

			if let Some(tile_cost) = grid.get(x, y) {
				if (x, y) == end {
					return Ok(cost + tile_cost);
				}
				
				priority_queue.push((Reverse(cost + tile_cost), x, y));
			}
		}
	}

	Err(anyhow::anyhow!("No path found"))
}

advent::problem!(
	r#"
		1163751742
		1381373672
		2136511328
		3694931569
		7463417111
		1319128137
		1359912421
		3125421639
		1293138521
		2311944581
	"#,
	40,
	315,
);
