use std::cmp::Reverse;

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

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
	let input: HashMap<(isize, isize), u32> = input
		.lines()
		.enumerate()
		.flat_map(|(x, line)| {
			line.chars().enumerate().map(move |(y, tile)| {
				(
					(x.try_into().unwrap(), y.try_into().unwrap()),
					tile.to_digit(10).unwrap(),
				)
			})
		})
		.collect();

	let width = input.keys().map(|(x, _)| *x).max().unwrap() + 1;
	let height = input.keys().map(|(_, y)| *y).max().unwrap() + 1;

	let grid: HashMap<(isize, isize), u32> = (0..5)
		.cartesian_product(0..5)
		.flat_map(|(x, y): (isize, isize)| {
			input.iter().map(move |((i, j), &cost)| {
				let x_cost: u32 = x.try_into().unwrap();
				let y_cost: u32 = y.try_into().unwrap();

				(
					(x * width + i, y * height + j),
					(cost + x_cost + y_cost - 1) % 9 + 1,
				)
			})
		})
		.collect();

	let mut priority_queue = std::collections::BinaryHeap::new();
	let mut visited = HashSet::new();

	priority_queue.push((Reverse(0), (0, 0)));
	let end = (
		grid.keys().map(|(x, _)| *x).max().unwrap(),
		grid.keys().map(|(_, y)| *y).max().unwrap(),
	);

	while let Some((Reverse(cost), (x, y))) = priority_queue.pop() {
		if !visited.insert((x, y)) {
			continue;
		}

		for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
			let next = (x + dx, y + dy);

			if let Some(tile_cost) = grid.get(&next) {
				if next == end {
					return Ok(cost + tile_cost);
				}

				priority_queue.push((Reverse(cost + tile_cost), next));
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
